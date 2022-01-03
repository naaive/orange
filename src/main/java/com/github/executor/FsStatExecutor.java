package com.github.executor;

import com.github.FileMsg;
import com.github.accessor.DbAccessor;
import com.github.accessor.FileDoc;
import com.github.accessor.IndexAccessor;
import com.github.accessor.StatProcess;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.nio.file.*;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Arrays;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Collectors;

/**
 * @author jeff
 */
@Slf4j
public class FsStatExecutor implements Runnable {

    private static final int COMMIT_THRESHOLD = 1000;
    private final String monitorPath;
    private final Set<String> excludePaths;
    private int addCnt;
    private DbAccessor dbAccessor;
    private IndexAccessor indexAccessor;
    private boolean aBoolean;


    public FsStatExecutor(String monitorPath, String[] excludePaths,DbAccessor dbAccessor,IndexAccessor indexAccessor) {
        this.monitorPath = monitorPath;
        this.excludePaths = Arrays.stream(excludePaths).collect(Collectors.toSet());
        this.dbAccessor = dbAccessor;
        this.indexAccessor = indexAccessor;
    }


    @SneakyThrows
    @Override
    public void run() {
        StatProcess statProcess = dbAccessor.fetchStatProcess();
//        if (statProcess!=null&& statProcess.getStatus()) {
//
//        }

//        travelFiles();
//        log.info("commit {} file(s) to index", addCnt);
//        addCnt = 0;
    }

    private void travelFiles() throws IOException {
        Files.walkFileTree(Paths.get(monitorPath), new FileVisitor<Path>() {
            @Override
            public FileVisitResult preVisitDirectory(Path dir, BasicFileAttributes attrs) throws IOException {
                String absPath = dir.toAbsolutePath().toString();

                if (excludePaths.contains(absPath)) {
                    log.info("skip dir:{} due to in the exclude paths:{}", absPath, excludePaths);
                    return FileVisitResult.SKIP_SUBTREE;
                }

                long modifiedTime = attrs.lastModifiedTime().toMillis();
                if (hasDocAndNoExpired(absPath, modifiedTime)) {
                    log.info("skip dir:{} due to no modification", absPath);
                    return FileVisitResult.SKIP_SUBTREE;
                }

                addDoc(attrs, absPath, true);
                return FileVisitResult.CONTINUE;
            }

            @Override
            public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) throws IOException {
                String absPath = file.toAbsolutePath().toString();
                addDoc(attrs, absPath, false);
                return FileVisitResult.CONTINUE;
            }

            @Override
            public FileVisitResult visitFileFailed(Path file, IOException exc) throws IOException {
                return FileVisitResult.CONTINUE;
            }

            @Override
            public FileVisitResult postVisitDirectory(Path dir, IOException exc) throws IOException {
                return FileVisitResult.CONTINUE;
            }
        });
    }

    @SneakyThrows
    private boolean hasDocAndNoExpired(String absPath, long modifiedTime) {
        Optional<FileMsg.File> optionalFile = dbAccessor.get(absPath);
        return optionalFile.filter(file -> file.getModifiedAt() == modifiedTime).isPresent();
    }

    @SneakyThrows
    private void addDoc(BasicFileAttributes attrs, String absPath, boolean isDir) {
        indexAccessor.add(new FileDoc()
                .setAbsPath(absPath)
                .setIsDir(isDir ? 1 : 0)
                .setIsSymbolicLink(attrs.isSymbolicLink() ? 1 : 0));

        dbAccessor.put(absPath, com.github.FileMsg.File.newBuilder()
                .setCreatedAt(attrs.creationTime().toMillis())
                .setModifiedAt(attrs.lastModifiedTime().toMillis())
                .setSize(attrs.size()).build());

        addCnt++;
        if (addCnt % COMMIT_THRESHOLD == 0) {
            log.info("commit {} file(s) to index", addCnt);
            addCnt = 0;
        }
    }
}

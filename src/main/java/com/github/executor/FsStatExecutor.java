package com.github.executor;

import com.github.FileMsg;
import com.github.accessor.DbAccessor;
import com.github.accessor.FileDoc;
import com.github.accessor.FileDocSuggester;
import com.github.accessor.IndexAccessor;
import com.github.utils.FileUtil;
import com.github.utils.ProcessUtil;
import io.netty.channel.DefaultEventLoopGroup;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.nio.file.*;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Arrays;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.concurrent.TimeUnit;
import java.util.stream.Collectors;

/**
 * @author jeff
 */
@Slf4j
public class FsStatExecutor implements Runnable {

    private static final int COMMIT_THRESHOLD = 10000;
    private final String monitorPath;
    private final Set<String> excludePaths;
    private final Set<String> excludeNames;
    private final DbAccessor dbAccessor;
    private final IndexAccessor indexAccessor;
    private final FileDocSuggester fileDocSuggester;
    private final DefaultEventLoopGroup executors;
    private int addCnt;

    public FsStatExecutor(
            String monitorPath,
            String[] excludePaths,
            Set<String> excludeNames,
            DbAccessor dbAccessor,
            IndexAccessor indexAccessor,
            FileDocSuggester fileDocSuggester,
            DefaultEventLoopGroup executors) {
        this.monitorPath = monitorPath;
        this.excludePaths = Arrays.stream(excludePaths).collect(Collectors.toSet());
        this.excludeNames = excludeNames;
        this.dbAccessor = dbAccessor;
        this.indexAccessor = indexAccessor;
        this.fileDocSuggester = fileDocSuggester;
        this.executors = executors;
    }

    @SneakyThrows
    @Override
    public void run() {

        if (ProcessUtil.shouldStat()) {
            log.info("no need to travel {} because of system load", monitorPath);
            return;
        }
        executors.scheduleAtFixedRate(
                () -> {
                    log.info(" commit {} file(s) to index", addCnt);

                    addCnt = 0;
                    indexAccessor.commit();
                },
                5,
                5,
                TimeUnit.SECONDS);

        log.info("start travel {} recursively", monitorPath);
        travelFiles();
    }

    private void travelFiles() throws IOException {
        Files.walkFileTree(Paths.get(monitorPath), new FileVisitor<>() {
            @Override
            public FileVisitResult preVisitDirectory(Path dir, BasicFileAttributes attrs) throws IOException {
                String absPath = dir.toAbsolutePath().toString();
                String name = FileUtil.absPath2name(absPath);

                if (excludeNames.stream().anyMatch(x -> Objects.equals(x, name))
                        || excludePaths.stream().anyMatch(x -> Objects.equals(absPath, x))) {

                    log.info("skip dir:{} due to in the exclude paths:{}", absPath, excludePaths);
                    return FileVisitResult.SKIP_SUBTREE;
                }

                addDoc(attrs, absPath, true);
                try {
                    Thread.sleep(5);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
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
        String name = FileUtil.absPath2name(absPath);
        String ext = FileUtil.name2ext(name);
        indexAccessor.add(new FileDoc()
                .setName(name)
                .setExt(ext)
                .setAbsPath(absPath)
                .setAbsPathIndexed(FileUtil.absPath2absPathIndexed(absPath))
                .setCreatedAt(attrs.creationTime().toMillis())
                .setModifiedAt(attrs.lastModifiedTime().toMillis())
                .setSize(attrs.size())
                .setIsDir(isDir ? 1 : 0)
                .setIsSymbolicLink(attrs.isSymbolicLink() ? 1 : 0));
        fileDocSuggester.put(name);
        dbAccessor.put(
                absPath,
                com.github.FileMsg.File.newBuilder()
                        .setCreatedAt(attrs.creationTime().toMillis())
                        .setModifiedAt(attrs.lastModifiedTime().toMillis())
                        .setSize(attrs.size())
                        .build());

        addCnt++;
        if (addCnt % COMMIT_THRESHOLD == 0) {
            log.info("commit {} file(s) to index", addCnt);
            addCnt = 0;
            indexAccessor.commit();
        }
    }
}

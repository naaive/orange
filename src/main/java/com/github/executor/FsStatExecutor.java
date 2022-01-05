package com.github.executor;

import com.github.FileMsg;
import com.github.accessor.DbAccessor;
import com.github.accessor.FileDoc;
import com.github.accessor.IndexAccessor;
import com.github.conf.IndexConf;
import com.github.utils.FileUtil;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.nio.file.*;
import java.nio.file.attribute.BasicFileAttributes;
import java.time.Duration;
import java.time.Instant;
import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.Optional;
import java.util.Set;
import java.util.TimeZone;
import java.util.stream.Collectors;

import static com.github.conf.IndexConf.readFromFile;

/**
 * @author jeff
 */
@Slf4j
public class FsStatExecutor implements Runnable {

    private static final int COMMIT_THRESHOLD = 100000;
    private final String monitorPath;
    private final Set<String> excludePaths;
    private final DbAccessor dbAccessor;
    private final IndexAccessor indexAccessor;
    private final IndexConf indexConf = readFromFile();
    private int addCnt;

    public FsStatExecutor(
            String monitorPath, String[] excludePaths, DbAccessor dbAccessor, IndexAccessor indexAccessor) {
        this.monitorPath = monitorPath;
        this.excludePaths = Arrays.stream(excludePaths).collect(Collectors.toSet());
        this.dbAccessor = dbAccessor;
        this.indexAccessor = indexAccessor;
    }

    @SneakyThrows
    @Override
    public void run() {
        LocalDateTime lastStatTime = LocalDateTime.ofInstant(
                Instant.ofEpochSecond(indexConf.getLastStatTime()),
                TimeZone.getDefault().toZoneId());
        LocalDateTime now = LocalDateTime.now();
        if (Duration.between(lastStatTime, now).toHours() < 1) {
            log.info("no need to travel {} recursively", monitorPath);
            return;
        }
        log.info("start travel {} recursively", monitorPath);
        travelFiles();
        log.info("commit {} file(s) to index", addCnt);
        addCnt = 0;
    }

    private void travelFiles() throws IOException {
        Files.walkFileTree(Paths.get(monitorPath), new FileVisitor<>() {
            @Override
            public FileVisitResult preVisitDirectory(Path dir, BasicFileAttributes attrs) throws IOException {
                String absPath = dir.toAbsolutePath().toString();

                if (excludePaths.stream().anyMatch(absPath::contains)
                        || excludePaths.stream().anyMatch(x -> x.contains(absPath))) {

                    log.info("skip dir:{} due to in the exclude paths:{}", absPath, excludePaths);
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
        }
    }
}

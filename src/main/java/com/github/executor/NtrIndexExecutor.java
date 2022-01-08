package com.github.executor;

import com.github.FileMsg;
import com.github.accessor.DbAccessor;
import com.github.accessor.FileDoc;
import com.github.accessor.FileDocSuggester;
import com.github.accessor.IndexAccessor;
import com.github.fshook.Cmd;
import com.github.fshook.FsEventQ;
import com.github.fshook.FsLog;
import com.github.utils.FileUtil;
import io.netty.channel.DefaultEventLoopGroup;
import lombok.extern.slf4j.Slf4j;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Arrays;
import java.util.List;
import java.util.Set;
import java.util.concurrent.TimeUnit;
import java.util.stream.Collectors;

@Slf4j
public class NtrIndexExecutor implements Runnable {
    private static final int COMMIT_THRESHOLD = 10000;
    private final DbAccessor dbAccessor;
    private final IndexAccessor indexAccessor;
    private final FileDocSuggester fileDocSuggester;
    private final DefaultEventLoopGroup executors;
    private final Set<String> excludePaths;
    private int addCnt;

    public NtrIndexExecutor(
            DbAccessor dbAccessor,
            IndexAccessor indexAccessor,
            FileDocSuggester fileDocSuggester, DefaultEventLoopGroup executors,
            Set<String> excludePaths) {

        this.dbAccessor = dbAccessor;
        this.indexAccessor = indexAccessor;
        this.fileDocSuggester = fileDocSuggester;
        this.executors = executors;
        this.excludePaths = excludePaths;
    }

    public void initialize() {

        executors.scheduleAtFixedRate(
                () -> {
                    log.info(" commit {} file(s) to index", addCnt);

                    addCnt = 0;
                    indexAccessor.commit();
                },
                5,
                5,
                TimeUnit.SECONDS);
    }

    @Override
    public void run() {
        FsEventQ q = new FsEventQ(
                Arrays.stream(File.listRoots()).map(File::getAbsolutePath).toArray(String[]::new));

        //noinspection InfiniteLoopStatement
        while (true) {
            try {
                doWork(q);
            } catch (Throwable e) {
                log.error("ntr indexing err", e);
            }
        }
    }

    private void doWork(FsEventQ q) {
        List<FsLog> fsLogs = q.poll(24).stream()
                .filter(x -> !x.getPath().contains("$RECYCLE.BIN"))
                .collect(Collectors.toList());
        log.debug("sync {} to index", fsLogs);
        for (FsLog fsLog : fsLogs) {
            Cmd cmd = fsLog.getCmd();
            String absPath = fsLog.getPath();

            if (excludePaths.stream().anyMatch(absPath::contains)
                    || excludePaths.stream().anyMatch(x -> x.contains(absPath))) {
                continue;
            }

            if (cmd == Cmd.D) {
                dbAccessor.del(absPath);
                indexAccessor.del(absPath);
            } else {
                BasicFileAttributes attrs;
                try {
                    attrs = Files.readAttributes(Paths.get(absPath), BasicFileAttributes.class);
                } catch (IOException e) {
                    continue;
                }
                String name = FileUtil.absPath2name(absPath);
                String ext = FileUtil.name2ext(name);
                indexAccessor.add(new FileDoc()
                        .setName(name)
                        .setExt(ext)
                        .setSize(attrs.size())
                        .setCreatedAt(attrs.creationTime().toMillis())
                        .setModifiedAt(attrs.lastModifiedTime().toMillis())
                        .setAbsPathIndexed(FileUtil.absPath2absPathIndexed(absPath))
                        .setAbsPath(absPath)
                        .setIsDir(attrs.isDirectory() ? 1 : 0)
                        .setIsSymbolicLink(attrs.isSymbolicLink() ? 1 : 0));
                addCnt++;
                if (addCnt % COMMIT_THRESHOLD == 0) {
                    indexAccessor.commit();
                    log.info("commit {} file(s) to index", addCnt);
                    addCnt = 0;
                }

                fileDocSuggester.put(name);

                dbAccessor.put(
                        absPath,
                        FileMsg.File.newBuilder()
                                .setModifiedAt(attrs.lastModifiedTime().toMillis())
                                .setSize(attrs.size())
                                .setCreatedAt(attrs.creationTime().toMillis())
                                .build());
            }
        }
    }
}

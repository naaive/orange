package com.github.executor;

import com.github.FileMsg;
import com.github.accessor.DbAccessor;
import com.github.accessor.FileDoc;
import com.github.accessor.IndexAccessor;
import com.github.fshook.Cmd;
import com.github.fshook.FsEventQ;
import com.github.fshook.FsLog;
import lombok.extern.slf4j.Slf4j;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

@Slf4j
public class NtrIndexExecutor implements Runnable {
    private final DbAccessor dbAccessor;
    private final IndexAccessor indexAccessor;

    public NtrIndexExecutor(DbAccessor dbAccessor, IndexAccessor indexAccessor) {

        this.dbAccessor = dbAccessor;
        this.indexAccessor = indexAccessor;
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
        List<FsLog> fsLogs = q.poll(1).stream()
                .filter(x -> !x.getPath().contains("$RECYCLE.BIN"))
                .collect(Collectors.toList());
        log.info("sync {} to index",fsLogs);
        for (FsLog fsLog : fsLogs) {
            Cmd cmd = fsLog.getCmd();
            String absPath = formatPath(fsLog.getPath());

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
                dbAccessor.put(
                        absPath,
                        FileMsg.File.newBuilder()
                                .setModifiedAt(attrs.lastModifiedTime().toMillis())
                                .setSize(attrs.size())
                                .setCreatedAt(attrs.creationTime().toMillis())
                                .build());
                indexAccessor.add(new FileDoc()
                        .setAbsPath(absPath)
                        .setIsDir(attrs.isDirectory() ? 1 : 0)
                        .setIsSymbolicLink(attrs.isSymbolicLink() ? 1 : 0));
            }
        }
    }

    private String formatPath(String logPath) {
        return Paths.get(logPath).normalize().toAbsolutePath().toString();
    }
}

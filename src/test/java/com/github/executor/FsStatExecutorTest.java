package com.github.executor;

import com.github.accessor.DbAccessor;
import com.github.accessor.IndexAccessor;
import com.github.conf.IndexConf;
import io.netty.channel.DefaultEventLoopGroup;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.util.Arrays;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.LockSupport;

import static com.github.conf.IndexConf.DATA_PATH;
import static com.github.conf.IndexConf.INDEX_PATH;
import static org.junit.jupiter.api.Assertions.*;

class FsStatExecutorTest {
    private final DefaultEventLoopGroup executors = new DefaultEventLoopGroup(4);

    @Test
    void run() {
        DbAccessor dbAccessor = new DbAccessor(DATA_PATH);
        IndexAccessor indexAccessor = new IndexAccessor(INDEX_PATH, dbAccessor, executors);
        Arrays.stream(File.listRoots()).map(x -> new FsStatExecutor(
                x.getAbsolutePath(),
                new String[]{"C:\\Users\\Administrator\\WebstormProjects\\untitled\\node_modules"},
                dbAccessor,
                indexAccessor)).forEach(x -> {

            executors.scheduleAtFixedRate(x, 0, 1, TimeUnit.DAYS);
        });

        LockSupport.park();


    }
}
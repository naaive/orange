package com.github.executor;

import com.github.accessor.DbAccessor;
import com.github.accessor.FileDocSuggester;
import com.github.accessor.IndexAccessor;
import io.netty.channel.DefaultEventLoopGroup;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.util.Arrays;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.LockSupport;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static com.github.conf.AppConf.DATA_PATH;
import static com.github.conf.AppConf.INDEX_PATH;

class FsStatExecutorTest {
    private final DefaultEventLoopGroup executors = new DefaultEventLoopGroup(4);

    @Test
    void run() {
        DbAccessor dbAccessor = new DbAccessor(DATA_PATH);
        IndexAccessor indexAccessor = new IndexAccessor(INDEX_PATH, dbAccessor, executors);
        Arrays.stream(File.listRoots()).map(x -> new FsStatExecutor(
                x.getAbsolutePath(),
                new String[]{"C:\\Users\\Administrator\\WebstormProjects\\untitled\\node_modules"},
                Stream.of("node_modules").collect(Collectors.toSet()), dbAccessor,
                indexAccessor, new FileDocSuggester(executors), executors)).forEach(x -> {

            executors.scheduleAtFixedRate(x, 0, 1, TimeUnit.DAYS);
        });

        LockSupport.park();


    }
}
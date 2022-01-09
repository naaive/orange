package com.github;

import com.github.accessor.DbAccessor;
import com.github.accessor.FileDocSuggester;
import com.github.accessor.IndexAccessor;
import com.github.conf.LogConf;
import com.github.executor.FsStatExecutor;
import com.github.executor.NtrIndexExecutor;
import com.github.handler.OrangeInitializer;
import com.github.utils.OsUtil;
import com.github.utils.ProcessUtil;
import io.netty.bootstrap.ServerBootstrap;
import io.netty.channel.Channel;
import io.netty.channel.ChannelOption;
import io.netty.channel.DefaultEventLoopGroup;
import io.netty.channel.EventLoopGroup;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.handler.logging.LogLevel;
import io.netty.handler.logging.LoggingHandler;
import lombok.extern.java.Log;

import java.io.File;
import java.util.Arrays;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static com.github.conf.IndexConf.*;

@Log
public class OrangeServer {

    private final DefaultEventLoopGroup executors = new DefaultEventLoopGroup(5);
    private DbAccessor dbAccessor;
    private IndexAccessor indexAccessor;
    private final String WINDOWS_PATH = "C:\\Windows";
    private final FileDocSuggester fileDocSuggester = new FileDocSuggester();


    static {
        LogConf.initialize();

    }

    public static void main(String[] args) {

                System.setProperty("project.path", "C:\\Users\\Administrator\\IdeaProjects\\github\\orange\\dist");
        ProcessUtil.cleanOrangeCore();
        if (OsUtil.isWindows()) {
            ProcessUtil.winKillByPort(PORT);
        }

        new OrangeServer().start();
    }

    private void start() {

        this.dbAccessor = new DbAccessor(DATA_PATH);
        this.indexAccessor = new IndexAccessor(INDEX_PATH, dbAccessor, executors);

        // Configure the server.
        EventLoopGroup bossGroup = new NioEventLoopGroup(1);
        EventLoopGroup workerGroup = new NioEventLoopGroup();
        try {
            ServerBootstrap b = new ServerBootstrap();
            b.option(ChannelOption.SO_BACKLOG, 1024);
            b.childOption(ChannelOption.TCP_NODELAY, true);
            b.group(bossGroup, workerGroup)
                    .channel(NioServerSocketChannel.class)
                    .handler(new LoggingHandler(String.valueOf(LogLevel.INFO)))
                    .childHandler(new OrangeInitializer(indexAccessor, fileDocSuggester));

            Channel ch = b.bind(PORT).sync().channel();

            System.err.println("Open your web browser and navigate to " + ("http") + "://127.0.0.1:" + PORT + '/');

            runTasks();

            Runtime.getRuntime().addShutdownHook(new Thread(ProcessUtil::clean));

            ch.closeFuture().sync();
        } catch (InterruptedException e) {
            log.log(Level.SEVERE, "start server err", e);
        } finally {
            bossGroup.shutdownGracefully();
            workerGroup.shutdownGracefully();
            executors.shutdownGracefully();
        }
    }

    private void runTasks() {

        //        FileSystemView fsv = FileSystemView.getFileSystemView();
        Arrays.stream(File.listRoots())
                .map(x -> new FsStatExecutor(
                        x.getAbsolutePath(),
                        new String[] {WINDOWS_PATH, "C:\\Users\\Administrator\\WebstormProjects\\untitled\\node_modules"
                        },
                        Stream.of("node_modules").collect(Collectors.toSet()),
                        dbAccessor,
                        indexAccessor,
                        fileDocSuggester,
                        executors))
                .forEach(x -> {
                    executors.scheduleAtFixedRate(x, 0, 1, TimeUnit.DAYS);
                });

        executors.submit(new NtrIndexExecutor(
                dbAccessor,
                indexAccessor,
                fileDocSuggester,
                executors,
                Stream.of(WINDOWS_PATH, PROJECT_PATH).collect(Collectors.toSet())));
    }
}

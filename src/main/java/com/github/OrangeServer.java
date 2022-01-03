package com.github;

import ch.qos.logback.classic.Level;
import com.github.accessor.DbAccessor;
import com.github.accessor.IndexAccessor;
import com.github.executor.FsStatExecutor;
import com.github.executor.NtrIndexExecutor;
import com.github.handler.OrangeInitializer;
import io.netty.bootstrap.ServerBootstrap;
import io.netty.channel.Channel;
import io.netty.channel.ChannelOption;
import io.netty.channel.DefaultEventLoopGroup;
import io.netty.channel.EventLoopGroup;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.handler.logging.LogLevel;
import io.netty.handler.logging.LoggingHandler;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.TimeUnit;

@Slf4j
public class OrangeServer {

    public static final String indexPath =
            "C:\\Users\\Administrator\\IdeaProjects\\orange\\src\\main\\resources\\.orange\\index";
    static final int PORT = Integer.parseInt(System.getProperty("port", "8080"));
    private static final String dataPath =
            "C:\\Users\\Administrator\\IdeaProjects\\orange\\src\\main\\resources\\.orange\\data";

    static {
        ch.qos.logback.classic.Logger rootLogger =
                (ch.qos.logback.classic.Logger) LoggerFactory.getLogger(Logger.ROOT_LOGGER_NAME);
        rootLogger.setLevel(Level.DEBUG);
    }

    private final String monitorPath = "C:\\Users\\Administrator\\WebstormProjects\\untitled";
    private final DefaultEventLoopGroup executors = new DefaultEventLoopGroup(4);
    private DbAccessor dbAccessor;
    private IndexAccessor indexAccessor;

    public static void main(String[] args) {
        System.setProperty("project.path", "C:\\Users\\Administrator\\IdeaProjects\\github\\orange\\dist");

        new OrangeServer().start();
    }

    @SneakyThrows
    private void start() {

        this.dbAccessor = new DbAccessor(dataPath);
        this.indexAccessor = new IndexAccessor(indexPath, dbAccessor);
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
                    .childHandler(new OrangeInitializer(indexAccessor));

            Channel ch = b.bind(PORT).sync().channel();

            System.err.println("Open your web browser and navigate to " + ("http") + "://127.0.0.1:" + PORT + '/');

            runTasks();

            ch.closeFuture().sync();
        } finally {
            bossGroup.shutdownGracefully();
            workerGroup.shutdownGracefully();
        }
    }

    private void runTasks() {
        FsStatExecutor statExecutor = new FsStatExecutor(
                monitorPath,
                new String[]{"C:\\Users\\Administrator\\WebstormProjects\\untitled\\node_modules"},
                dbAccessor,
                indexAccessor);
        executors.scheduleAtFixedRate(statExecutor, 0, 2, TimeUnit.SECONDS);
        executors.submit(new NtrIndexExecutor(dbAccessor, indexAccessor));
    }
}

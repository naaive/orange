package com.github.handler;

import com.alibaba.fastjson.JSON;
import com.github.accessor.FileDocSuggester;
import com.github.accessor.FileView;
import com.github.accessor.IndexAccessor;
import io.netty.buffer.ByteBuf;
import io.netty.channel.ChannelFuture;
import io.netty.channel.ChannelFutureListener;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.handler.codec.http.DefaultFullHttpResponse;
import io.netty.handler.codec.http.FullHttpRequest;
import io.netty.handler.codec.http.FullHttpResponse;
import io.netty.handler.codec.http.HttpUtil;
import lombok.SneakyThrows;

import java.awt.*;
import java.io.File;
import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLDecoder;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Objects;

import static io.netty.handler.codec.http.HttpHeaderNames.*;
import static io.netty.handler.codec.http.HttpHeaderValues.CLOSE;
import static io.netty.handler.codec.http.HttpHeaderValues.TEXT_PLAIN;
import static io.netty.handler.codec.http.HttpResponseStatus.OK;

public class OrangeHandler extends SimpleChannelInboundHandler<FullHttpRequest> {
    private static final String SEARCH_PATH = "/q";
    private static final String SUGGEST_PATH = "/sg";
    private static final String OPEN_FOLDER_PATH = "/ofd";
    private final IndexAccessor indexAccessor;
    private final FileDocSuggester fileDocSuggester;

    public OrangeHandler(IndexAccessor indexAccessor, FileDocSuggester fileDocSuggester) {
        this.indexAccessor = indexAccessor;
        this.fileDocSuggester = fileDocSuggester;
    }

    @Override
    public void channelReadComplete(ChannelHandlerContext ctx) {
        ctx.flush();
    }

    @SneakyThrows
    @Override
    public void channelRead0(ChannelHandlerContext ctx, FullHttpRequest msg) {

        URL url = genUrl(msg.uri());

        if (Objects.equals(url.getPath(), SEARCH_PATH)) {
            doSearch(ctx, msg, url);
        }

        if (Objects.equals(url.getPath(), SUGGEST_PATH)) {
            doSuggest(ctx, msg, url);
        }

        if (Objects.equals(url.getPath(), OPEN_FOLDER_PATH)) {
            doOpenFolder(ctx, msg, url);
        }
    }

    @SneakyThrows
    private void doOpenFolder(ChannelHandlerContext ctx, FullHttpRequest msg, URL url) {
        String query = url.getQuery();
        String[] split = query.split("=");
        String folder = split[1];

        File file = new File(URLDecoder.decode(folder, StandardCharsets.UTF_8));
        while (true) {
            if (file.exists()) {
                if (file.isFile()) {
                    file = file.getParentFile();
                } else {
                    Desktop.getDesktop().open(file);
                    break;
                }
            }
        }
        doResponse(ctx, msg, JSON.toJSONString(file.getAbsoluteFile()));
    }

    private void doSuggest(ChannelHandlerContext ctx, FullHttpRequest msg, URL url) {
        String query = url.getQuery();
        String[] split = query.split("=");
        String keyword = split[1];
        List<String> lookup = fileDocSuggester.lookup(keyword);
        doResponse(ctx, msg, JSON.toJSONString(lookup));
    }

    private void doResponse(ChannelHandlerContext ctx, FullHttpRequest msg, String s) {
        boolean keepAlive = HttpUtil.isKeepAlive(msg);
        ByteBuf byteBuf = ctx.alloc().buffer().writeBytes(s.getBytes(StandardCharsets.UTF_8));
        FullHttpResponse response = new DefaultFullHttpResponse(msg.protocolVersion(), OK, byteBuf);
        response.headers()
                .set(CONTENT_TYPE, TEXT_PLAIN)
                .setInt(CONTENT_LENGTH, response.content().readableBytes());

        if (keepAlive) {
            if (!msg.protocolVersion().isKeepAliveDefault()) {
                response.headers().set(CONNECTION, KEEP_ALIVE);
            }
        } else {
            // Tell the client we're going to close the connection.
            response.headers().set(CONNECTION, CLOSE);
        }
        ChannelFuture f = ctx.write(response);

        if (!keepAlive) {
            f.addListener(ChannelFutureListener.CLOSE);
        }
    }

    private void doSearch(ChannelHandlerContext ctx, FullHttpRequest msg, URL url) {
        String query = url.getQuery();
        String[] split = query.split("=");
        String keyword = split[1];
        List<FileView> docs = indexAccessor.search(keyword);
        doResponse(ctx, msg, JSON.toJSONString(docs));
    }

    private URL genUrl(String uri) throws MalformedURLException {
        return new URL("http://localhost" + uri);
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        cause.printStackTrace();
        ctx.close();
    }
}

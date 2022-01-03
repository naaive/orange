package com.github.handler;

import com.alibaba.fastjson.JSON;
import com.github.accessor.FileView;
import com.github.accessor.IndexAccessor;
import io.netty.buffer.ByteBuf;
import io.netty.channel.*;
import io.netty.handler.codec.http.*;
import lombok.SneakyThrows;

import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Objects;

import static io.netty.handler.codec.http.HttpHeaderNames.*;
import static io.netty.handler.codec.http.HttpHeaderValues.CLOSE;
import static io.netty.handler.codec.http.HttpHeaderValues.TEXT_PLAIN;
import static io.netty.handler.codec.http.HttpResponseStatus.OK;

public class OrangeHandler extends SimpleChannelInboundHandler<HttpObject> {
    private final IndexAccessor indexAccessor;

    public OrangeHandler(IndexAccessor indexAccessor) {
        this.indexAccessor = indexAccessor;
    }

    @Override
    public void channelReadComplete(ChannelHandlerContext ctx) {
        ctx.flush();
    }

    @SneakyThrows
    @Override
    public void channelRead0(ChannelHandlerContext ctx, HttpObject msg) {
        if (msg instanceof HttpRequest) {
            HttpRequest req = (HttpRequest) msg;
            boolean keepAlive = HttpUtil.isKeepAlive(req);
            URL url = new URL("http://localhost" + req.uri());
            FullHttpResponse response = genResp(ctx, req, url);

            response.headers()
                    .set(CONTENT_TYPE, TEXT_PLAIN)
                    .setInt(CONTENT_LENGTH, response.content().readableBytes());

            if (keepAlive) {
                if (!req.protocolVersion().isKeepAliveDefault()) {
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
    }

    private FullHttpResponse genResp(ChannelHandlerContext ctx, HttpRequest req, URL url) {
        if (Objects.equals(url.getPath(), "/q")) {
            String query = url.getQuery();
            String[] split = query.split("=");
            String keyword = split[1];
            List<FileView> docs = indexAccessor.search(keyword);
            ByteBuf byteBuf =
                    ctx.alloc().buffer().writeBytes(JSON.toJSONString(docs).getBytes(StandardCharsets.UTF_8));
            return new DefaultFullHttpResponse(req.protocolVersion(), OK, byteBuf);
        }
        return new DefaultFullHttpResponse(req.protocolVersion(), OK);
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        cause.printStackTrace();
        ctx.close();
    }
}

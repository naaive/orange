package com.github.handler;

import com.github.accessor.IndexAccessor;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.ChannelPipeline;
import io.netty.channel.socket.SocketChannel;
import io.netty.handler.codec.http.HttpServerCodec;
import io.netty.handler.codec.http.HttpServerExpectContinueHandler;


public class OrangeInitializer extends ChannelInitializer<SocketChannel> {

    private IndexAccessor indexAccessor;

    public OrangeInitializer(IndexAccessor indexAccessor) {
        this.indexAccessor = indexAccessor;
    }

    @Override
    public void initChannel(SocketChannel ch) {
        ChannelPipeline p = ch.pipeline();
        p.addLast(new HttpServerCodec());
        p.addLast(new HttpServerExpectContinueHandler());
        p.addLast(new OrangeHandler(indexAccessor));
    }
}

package com.github.handler;

import com.github.accessor.IndexAccessor;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.ChannelPipeline;
import io.netty.channel.socket.SocketChannel;
import io.netty.handler.codec.http.HttpObjectAggregator;
import io.netty.handler.codec.http.HttpServerCodec;
import io.netty.handler.stream.ChunkedWriteHandler;


public class OrangeInitializer extends ChannelInitializer<SocketChannel> {

    private final IndexAccessor indexAccessor;

    public OrangeInitializer(IndexAccessor indexAccessor) {
        this.indexAccessor = indexAccessor;
    }

    @Override
    public void initChannel(SocketChannel ch) {
        ChannelPipeline pipeline = ch.pipeline();

        pipeline.addLast(new HttpServerCodec());
        pipeline.addLast(new HttpObjectAggregator(65536));
        pipeline.addLast(new ChunkedWriteHandler());

        pipeline.addLast(new OrangeHandler(indexAccessor));
        pipeline.addLast(new StaticServerHandler());

    }
}

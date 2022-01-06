package com.github.handler;

import com.github.accessor.FileDocSuggester;
import com.github.accessor.IndexAccessor;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.ChannelPipeline;
import io.netty.channel.socket.SocketChannel;
import io.netty.handler.codec.http.HttpObjectAggregator;
import io.netty.handler.codec.http.HttpServerCodec;
import io.netty.handler.codec.http.cors.CorsConfig;
import io.netty.handler.codec.http.cors.CorsConfigBuilder;
import io.netty.handler.codec.http.cors.CorsHandler;
import io.netty.handler.stream.ChunkedWriteHandler;

public class OrangeInitializer extends ChannelInitializer<SocketChannel> {

    private final IndexAccessor indexAccessor;
    private final FileDocSuggester fileDocSuggester;

    public OrangeInitializer(IndexAccessor indexAccessor, FileDocSuggester fileDocSuggester) {
        this.indexAccessor = indexAccessor;
        this.fileDocSuggester = fileDocSuggester;
    }

    @Override
    public void initChannel(SocketChannel ch) {
        ChannelPipeline pipeline = ch.pipeline();

        pipeline.addLast(new HttpServerCodec());
        pipeline.addLast(new HttpObjectAggregator(65536));
        pipeline.addLast(new ChunkedWriteHandler());
        CorsConfig corsConfig = CorsConfigBuilder.forAnyOrigin()
                .allowNullOrigin()
                .allowCredentials()
                .build();
        pipeline.addLast(new CorsHandler(corsConfig));
        pipeline.addLast(new OrangeHandler(indexAccessor,fileDocSuggester));
    }
}

const express = require('express');
const { createProxyMiddleware } = require('http-proxy-middleware');
const cors = require('cors')
const app = express();
app.use(cors())

app.use('/api', createProxyMiddleware({
    target: 'http://127.0.0.1:8080',
    changeOrigin: true,
    pathRewrite: {
        '^/api' : '',
        // '^/api/old-path' : '/api/new-path', // 重写请求，请求api/old-path，会被解析为/api/new-path
        // '^/api/remove/path' : '/path'
    }
}));
app.listen(3001);
package com.github.accessor;

import com.alibaba.fastjson.JSON;
import com.github.FileMsg;
import lombok.SneakyThrows;
import org.iq80.leveldb.DB;
import org.iq80.leveldb.Options;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.util.Optional;

import static org.iq80.leveldb.impl.Iq80DBFactory.factory;

public class DbAccessor {

    private static DB db;

    public DbAccessor(String dataPath) {
        initialize(dataPath);
    }

    @SneakyThrows
    private void initialize(String dataPath) {
        Options options = new Options();
        options.createIfMissing(true);
         db = factory.open(new File(dataPath), options);
    }

    @SneakyThrows
    public Optional<FileMsg.File> get(String absPath) {
        byte[] bytes = db.get(absPath.getBytes(StandardCharsets.UTF_8));
        if (bytes == null) {
            return Optional.empty();
        }
        return Optional.of(com.github.FileMsg.File.parseFrom(bytes));
    }

    @SneakyThrows
    public synchronized void put(String absPath, FileMsg.File file) {
        db.put(absPath.getBytes(StandardCharsets.UTF_8), file.toByteArray());
    }

    @SneakyThrows
    public synchronized void del(String absPath) {
        db.delete(absPath.getBytes(StandardCharsets.UTF_8));
    }

    @SneakyThrows
    public synchronized void saveStatProcess(StatProcess process) {
        db.put(
                "process#stat#v1".getBytes(StandardCharsets.UTF_8),
                JSON.toJSONString(process).getBytes(StandardCharsets.UTF_8));
    }

    @SneakyThrows
    public StatProcess fetchStatProcess() {
        byte[] bytes = db.get("process#stat#v1".getBytes(StandardCharsets.UTF_8));
        return JSON.parseObject(bytes, StatProcess.class);
    }
}

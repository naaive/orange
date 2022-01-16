package com.github.accessor;

import com.github.FileMsg;
import com.github.utils.JsonUtil;
import com.google.protobuf.InvalidProtocolBufferException;
import lombok.extern.java.Log;
import org.iq80.leveldb.DB;
import org.iq80.leveldb.Options;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Optional;
import java.util.logging.Level;

import static org.iq80.leveldb.impl.Iq80DBFactory.factory;

@Log
public class DbAccessor {

    private static DB db;

    public DbAccessor(String dataPath) {
        initialize(dataPath);
    }

    private void initialize(String dataPath) {
        try {
            Options options = new Options();
            options.createIfMissing(true);
            db = factory.open(new File(dataPath), options);
        } catch (Exception e) {
            log.log(Level.SEVERE, "DbAccessor initialize err", e);
            Runtime.getRuntime().exit(-1);
        }
    }

    public Optional<FileMsg.File> get(String absPath) {
        try {
            byte[] bytes = db.get(absPath.getBytes(StandardCharsets.UTF_8));
            if (bytes == null) {
                return Optional.empty();
            }
            return Optional.of(FileMsg.File.parseFrom(bytes));
        } catch (InvalidProtocolBufferException e) {
            log.log(Level.SEVERE, "get err", e);
            return Optional.empty();
        }
    }

    public synchronized void put(String absPath, FileMsg.File file) {
        db.put(absPath.getBytes(StandardCharsets.UTF_8), file.toByteArray());
    }

    public synchronized void del(String absPath) {
        db.delete(absPath.getBytes(StandardCharsets.UTF_8));
    }

    public synchronized void saveStatProcess(StatProcess process) {
        db.put(
                "process#stat#v1".getBytes(StandardCharsets.UTF_8),

                JsonUtil.toJson(process).getBytes(StandardCharsets.UTF_8));
    }

    public StatProcess fetchStatProcess() {
        byte[] bytes = db.get("process#stat#v1".getBytes(StandardCharsets.UTF_8));
        return JsonUtil.fromJson(new String(bytes), StatProcess.class);
    }
}

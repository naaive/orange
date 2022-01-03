package com.github.fshook;

import lombok.Data;
import lombok.experimental.Accessors;

import java.nio.file.Paths;

@Data
@Accessors(chain = true)
public class FsLog {
    private Cmd cmd;
    private String path;

    public FsLog setPath(String path) {
        this.path = formatPath(path);
        return this;
    }

    public static String formatPath(String logPath) {
        return Paths.get(logPath).normalize().toAbsolutePath().toString();
    }
}

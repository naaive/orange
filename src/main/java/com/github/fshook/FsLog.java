package com.github.fshook;

import lombok.Data;
import lombok.experimental.Accessors;

import static com.github.utils.FileUtil.formatPath;

@Data
@Accessors(chain = true)
public class FsLog {
    private Cmd cmd;
    private String path;

    public FsLog setPath(String path) {
        this.path = formatPath(path);
        return this;
    }
}

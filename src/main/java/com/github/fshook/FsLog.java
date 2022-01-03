package com.github.fshook;

import lombok.Data;
import lombok.experimental.Accessors;

@Data
@Accessors(chain = true)
public class FsLog {
    private Cmd cmd;
    private String path;
}

package com.github.accessor;

import lombok.Data;
import lombok.experimental.Accessors;

@Data
@Accessors(chain = true)
public class FileView {
    private String absPath;
    private String ext;
    private String name;
    private int isDir;
    private int isSymbolicLink;
    private long createdAt;
    private long modifiedAt;
    private long size;
}

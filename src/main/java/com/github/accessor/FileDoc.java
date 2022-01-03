package com.github.accessor;

import lombok.Data;
import lombok.experimental.Accessors;
import org.apache.lucene.document.*;

@Data
@Accessors(chain = true)
public class FileDoc {
    public static final String IS_DIR = "isDir";
    public static final String ABS_PATH = "absPath";
    public static final String IS_SYMBOLICLINK = "isSymbolicLink";
    private String absPath;
    private int isDir;
    private int isSymbolicLink;

    public Document toDocument() {
        Document document = new Document();
        document.add(new TextField(ABS_PATH, absPath, Field.Store.YES));
        document.add(new IntPoint(IS_DIR, isDir));
        document.add(new StoredField(IS_DIR, isDir));
        document.add(new IntPoint(IS_SYMBOLICLINK, isSymbolicLink));
        document.add(new StoredField(IS_SYMBOLICLINK, isSymbolicLink));
        return document;
    }
}

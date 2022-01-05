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
    public static final String EXT = "ext";
    public static final String NAME = "name";
    public static final String CREATED_AT = "createdAt";
    public static final String MODIFIED_AT = "modifiedAt";
    public static final String SIZE = "size";
    private String absPath;
    private String ext;
    private String name;
    private int isDir;
    private int isSymbolicLink;
    private Long createdAt;
    private Long modifiedAt;
    private Long size;

    public Document toDocument() {
        Document document = new Document();
        document.add(new StringField(ABS_PATH, absPath, Field.Store.YES));

        document.add(new StringField(EXT, ext, Field.Store.YES));
        document.add(new TextField(NAME, name, Field.Store.YES));

        document.add(new IntPoint(IS_DIR, isDir));
        document.add(new StoredField(IS_DIR, isDir));

        document.add(new IntPoint(IS_SYMBOLICLINK, isSymbolicLink));
        document.add(new StoredField(IS_SYMBOLICLINK, isSymbolicLink));


        document.add(new LongPoint(CREATED_AT, createdAt));
        document.add(new StoredField(CREATED_AT, createdAt));

        document.add(new LongPoint(MODIFIED_AT, modifiedAt));
        document.add(new StoredField(MODIFIED_AT, modifiedAt));

        document.add(new LongPoint(SIZE, size));
        document.add(new StoredField(SIZE, size));
        return document;
    }
}

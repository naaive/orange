package com.github.accessor;

import com.github.utils.JsonUtil;
import org.apache.lucene.search.suggest.InputIterator;
import org.apache.lucene.util.BytesRef;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Iterator;
import java.util.List;
import java.util.Set;

public class FileDocIterator implements InputIterator {
    private final Iterator<FileDoc> fileDocsIter;
    private FileDoc next;

    public FileDocIterator(List<FileDoc> fileDocs) {
        this.fileDocsIter = fileDocs.iterator();
    }

    @Override
    public long weight() {
        return 1;
    }

    @Override
    public BytesRef payload() {
        return new BytesRef(JsonUtil.toJson(next).getBytes(StandardCharsets.UTF_8));
    }

    @Override
    public boolean hasPayloads() {
        return fileDocsIter.hasNext();
    }

    @Override
    public Set<BytesRef> contexts() {
        return null;
    }

    @Override
    public boolean hasContexts() {
        return false;
    }

    @Override
    public BytesRef next() throws IOException {
        if (fileDocsIter.hasNext()) {
            next = fileDocsIter.next();

            String name = next.getName();
            return new BytesRef(name);
        }
        next = null;
        return null;
    }
}

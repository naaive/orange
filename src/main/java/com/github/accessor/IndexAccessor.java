package com.github.accessor;


import com.github.FileMsg;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.*;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.ScoreDoc;
import org.apache.lucene.search.TopDocs;
import org.apache.lucene.store.FSDirectory;

import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;

@Slf4j
public class IndexAccessor {
    private final DbAccessor dbAccessor;
    private IndexSearcher indexSearcher;
    private IndexWriter indexWriter;
    private QueryParser parser;
    private int addCnt;
    private static final int COMMIT_THRESHOLD = 50;


    public IndexAccessor(String indexPath, DbAccessor dbAccessor) {
        this.dbAccessor = dbAccessor;
        initialize(indexPath);
    }

    @SneakyThrows
    private void initialize(String indexPath) {
        FSDirectory directory = FSDirectory.open(Paths.get(indexPath));
        indexWriter = new IndexWriter(directory, new IndexWriterConfig(new StandardAnalyzer()));
        indexWriter.commit();
        DirectoryReader reader = DirectoryReader.open(directory);
        indexSearcher = new IndexSearcher(reader);
        parser = new QueryParser(FileDoc.NAME, new StandardAnalyzer());
    }

    @SneakyThrows
    public synchronized void add(FileDoc fileDoc) {
        Optional<FileMsg.File> file = dbAccessor.get(fileDoc.getAbsPath());
        if (file.isPresent()) {
            return;
        }
        indexWriter.addDocument(fileDoc.toDocument());
        addCnt++;
        if (addCnt % COMMIT_THRESHOLD == 0) {
            indexWriter.commit();
            log.info("commit {} file(s) to index", addCnt);
            addCnt = 0;
        }
    }

    @SneakyThrows
    public List<FileView> search(String kw) {
        Query src = parser.parse(kw);
        TopDocs search = indexSearcher.search(src, 50);
        ScoreDoc[] docs = search.scoreDocs;
        return Arrays.stream(docs).map(this::buildFileView).collect(Collectors.toList());
    }

    @SneakyThrows
    public synchronized void del(String path) {
        indexWriter.deleteDocuments(new Term(FileDoc.ABS_PATH, path));
    }

    @SneakyThrows
    private FileView buildFileView(ScoreDoc x) {
        Document document = indexSearcher.doc(x.doc);
        List<IndexableField> fields = document.getFields();
        FileView view = new FileView();
        for (IndexableField field : fields) {
            String name = field.name();
            if (Objects.equals(name, FileDoc.NAME)) {
                view.setName(field.getCharSequenceValue().toString());
            }
            if (Objects.equals(name, FileDoc.ABS_PATH)) {
                view.setAbsPath(field.getCharSequenceValue().toString());
            }
            if (Objects.equals(name, FileDoc.CREATED_AT)) {
                view.setCreatedAt(Long.parseLong(field.getCharSequenceValue().toString()));
            }
            if (Objects.equals(name, FileDoc.MODIFIED_AT)) {
                view.setModifiedAt(Long.parseLong(field.getCharSequenceValue().toString()));
            }
            if (Objects.equals(name, FileDoc.SIZE)) {
                view.setSize(Long.parseLong(field.getCharSequenceValue().toString()));
            }
            if (Objects.equals(name, FileDoc.EXT)) {
                view.setExt(field.getCharSequenceValue().toString());
            }
            if (Objects.equals(name, FileDoc.IS_DIR)) {
                view.setIsDir(Integer.parseInt(field.getCharSequenceValue().toString()));
            }
            if (Objects.equals(name, FileDoc.IS_SYMBOLICLINK)) {
                view.setIsSymbolicLink(Integer.parseInt(field.getCharSequenceValue().toString()));
            }
        }

        return view;
    }
}

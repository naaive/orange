package com.github.accessor;


import com.github.FileMsg;
import lombok.SneakyThrows;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.index.Term;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.ScoreDoc;
import org.apache.lucene.search.TopDocs;
import org.apache.lucene.store.FSDirectory;

import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class IndexAccessor {
    private final DbAccessor dbAccessor;
    private IndexSearcher indexSearcher;
    private IndexWriter indexWriter;
    private QueryParser parser;


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
        parser = new QueryParser(FileDoc.ABS_PATH, new StandardAnalyzer());
    }

    @SneakyThrows
    public synchronized void add(FileDoc fileDoc) {
        indexWriter.addDocument(fileDoc.toDocument());
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
        int id = x.doc;
        Document
                document = indexSearcher.doc(id);

        int isDir = Integer.parseInt(document.getField(FileDoc.IS_DIR).getCharSequenceValue().toString());
        int isSymbolicLink = Integer.parseInt(document.getField(FileDoc.IS_SYMBOLICLINK).getCharSequenceValue().toString());

        String absPath = document.getField(FileDoc.ABS_PATH).getCharSequenceValue().toString();
        FileMsg.File file = dbAccessor.get(absPath).get();
        return new FileView().setAbsPath(absPath)
                .setIsDir(isDir)
                .setIsSymbolicLink(isSymbolicLink)
                .setCreatedAt(file.getCreatedAt())
                .setModifiedAt(file.getModifiedAt())
                .setSize(file.getSize());
    }
}

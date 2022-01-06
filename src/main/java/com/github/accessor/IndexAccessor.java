package com.github.accessor;

import com.github.FileMsg;
import io.netty.channel.DefaultEventLoopGroup;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.*;
import org.apache.lucene.search.*;
import org.apache.lucene.store.FSDirectory;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.ReentrantLock;
import java.util.stream.Collectors;

@Slf4j
public class IndexAccessor {
    private final DbAccessor dbAccessor;
    private final DefaultEventLoopGroup executors;
    private final ReentrantLock lock = new ReentrantLock();
    private volatile IndexSearcher indexSearcher;
    private volatile DirectoryReader reader;
    private IndexWriter indexWriter;

    public IndexAccessor(String indexPath, DbAccessor dbAccessor, DefaultEventLoopGroup executors) {
        this.dbAccessor = dbAccessor;
        this.executors = executors;
        initialize(indexPath);
    }

    @SneakyThrows
    private void initialize(String indexPath) {
        FSDirectory directory = FSDirectory.open(Paths.get(indexPath));
        indexWriter = new IndexWriter(directory, new IndexWriterConfig(new StandardAnalyzer()));
        indexWriter.commit();
        reader = DirectoryReader.open(directory);

        indexSearcher = new IndexSearcher(reader);

        executors.scheduleAtFixedRate(
                () -> {
                    log.info("rebuild index searcher");
                    try {
                        lock.lock();
                        try {
                            reader.close();
                            reader = DirectoryReader.open(directory);
                            indexSearcher = new IndexSearcher(reader);
                        } finally {
                            lock.unlock();
                        }
                    } catch (IOException e) {
                        e.printStackTrace();
                    }
                },
                5,
                5,
                TimeUnit.SECONDS);
    }

    @SneakyThrows
    public synchronized void add(FileDoc fileDoc) {
        Optional<FileMsg.File> file = dbAccessor.get(fileDoc.getAbsPath());
        if (file.isPresent()) {
            return;
        }
        indexWriter.addDocument(fileDoc.toDocument());
    }

    @SneakyThrows
    public void commit() {
        indexWriter.commit();
    }

    @SneakyThrows
    public List<FileView> search(String kw) {
        lock.lock();
        try {
            BooleanQuery query = new BooleanQuery.Builder()
                    .add(new BoostQuery(new TermQuery(new Term(FileDoc.NAME, kw)), 4), BooleanClause.Occur.SHOULD)
                    .add(
                            new BoostQuery(new TermQuery(new Term(FileDoc.ABS_PATH_INDEXED, kw)), 1),
                            BooleanClause.Occur.SHOULD)
                    .build();
            TopDocs search = indexSearcher.search(query, 1000);
            ScoreDoc[] docs = search.scoreDocs;
            return Arrays.stream(docs).map(this::buildFileView).collect(Collectors.toList());
        } finally {
            lock.unlock();
        }
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
                view.setIsSymbolicLink(
                        Integer.parseInt(field.getCharSequenceValue().toString()));
            }
        }

        return view;
    }
}

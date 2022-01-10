package com.github.accessor;

import com.github.FileMsg;
import com.github.ik.IKAnalyzer;
import io.netty.channel.DefaultEventLoopGroup;
import lombok.extern.java.Log;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.*;
import org.apache.lucene.queryparser.classic.ParseException;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.*;
import org.apache.lucene.store.FSDirectory;

import java.io.IOException;
import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.ReentrantLock;
import java.util.logging.Level;
import java.util.stream.Collectors;

@Log
public class IndexAccessor {
    private final DbAccessor dbAccessor;
    private final DefaultEventLoopGroup executors;
    private final ReentrantLock lock = new ReentrantLock();
    private volatile IndexSearcher indexSearcher;
    private volatile DirectoryReader reader;
    private volatile IndexWriter indexWriter;
    private final QueryParser absPathParser = new QueryParser(FileDoc.ABS_PATH_INDEXED, new IKAnalyzer(true));
    private final QueryParser nameParser = new QueryParser(FileDoc.NAME, new IKAnalyzer(true));

    public IndexAccessor(String indexPath, DbAccessor dbAccessor, DefaultEventLoopGroup executors) {
        this.dbAccessor = dbAccessor;
        this.executors = executors;
        initialize(indexPath);
    }

    private void initialize(String indexPath) {
        try {
            FSDirectory directory = FSDirectory.open(Paths.get(indexPath));
            indexWriter = new IndexWriter(directory, new IndexWriterConfig(new IKAnalyzer(false)));
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
                                indexWriter.close();
                                reader = DirectoryReader.open(directory);
                                indexSearcher = new IndexSearcher(reader);
                                indexWriter = new IndexWriter(directory, new IndexWriterConfig(new IKAnalyzer(false)));
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
        } catch (IOException e) {
            log.log(Level.SEVERE, "initialize err", e);
        }
    }

    public synchronized void add(FileDoc fileDoc) {
        lock.lock();
        try {
            // todo remove
            Optional<FileMsg.File> file = dbAccessor.get(fileDoc.getAbsPath());
            if (file.isPresent()) {
                return;
            }
            indexWriter.addDocument(fileDoc.toDocument());
        } catch (Exception e) {
            log.log(Level.SEVERE, "add err", e);
        } finally {
            lock.unlock();
        }
    }

    public void commit() {
        try {
            indexWriter.commit();
        } catch (IOException e) {
            log.log(Level.SEVERE, "commit err", e);
        }
    }

    public List<FileView> search(String kw) {
        lock.lock();
        try {
            String decode = URLDecoder.decode(kw, StandardCharsets.UTF_8);
            if (Objects.equals(decode, "*") || Objects.equals(decode, "")) {
                TopDocs search = indexSearcher.search(nameParser.parse("*:*"), 100);
                ScoreDoc[] docs = search.scoreDocs;
                return Arrays.stream(docs).map(this::buildFileView).collect(Collectors.toList());
            }
            Query absq = absPathParser.parse(decode);
            Query nameq = nameParser.parse(decode);
            BooleanQuery query = new BooleanQuery.Builder()
                    .add(new BoostQuery(absq, 1), BooleanClause.Occur.SHOULD)
                    .add(new BoostQuery(nameq, 4), BooleanClause.Occur.SHOULD)
                    .build();
            TopDocs search = indexSearcher.search(query, 100);
            ScoreDoc[] docs = search.scoreDocs;
            return Arrays.stream(docs).map(this::buildFileView).collect(Collectors.toList());
        } catch (ParseException | IOException e) {
            log.log(Level.SEVERE, "search err", e);
            return Collections.emptyList();
        } finally {
            lock.unlock();
        }
    }

    public synchronized void del(String path) {
        lock.lock();
        try {
            indexWriter.deleteDocuments(new Term(FileDoc.ABS_PATH, path));
        } catch (IOException e) {
            log.log(Level.SEVERE, "del  err", e);
        } finally {
            lock.unlock();
        }
    }

    private FileView buildFileView(ScoreDoc x) {
        Document document = null;
        try {
            document = indexSearcher.doc(x.doc);
        } catch (IOException e) {
            log.log(Level.SEVERE, "buildFileView err", e);
            throw new RuntimeException("buildFileView err");
        }
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

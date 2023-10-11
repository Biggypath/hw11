use std::fs;
fn main() {
    count_html()
}


fn make_document(text: &str) -> Vec<String> {
    text.split("\n\n").map(|s| s.to_string()).collect()
}

fn rank_documents(docs: &mut Vec<Vec<String>>) {
    docs.sort_by(|a, b| b.len().cmp(&a.len()));
}


#[test]
fn test_doc() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,\n\
        —\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
    ";

    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let mut docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];

    rank_documents(&mut docs);
    assert_eq!(docs, vec![doc3, doc2, doc1]);
}

fn count_paragraphs(document: &Vec<String>) -> usize {
    document.len()   
}

fn num_html() {
    let file_paths = vec!["abc.txt", "bustle.txt", "fox.txt", "empty.txt"];
    let mut documents: Vec<Vec<String>> = Vec::new();

    for path in &file_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let document = make_document(&content);
            documents.push(document);
        } else {
            println!("Error reading file: {}", path);
        }
    }

    rank_documents(&mut documents);

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Manage Paragraph</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Manage Paragraph</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>File</th>
                        <th>Paragraphs No.</th>
                    </tr>
                </thead>
                <tbody>");

    for document in documents.iter() {
        table.push_str("<tr>");
        for file in &file_paths {
            if let Ok(content) = fs::read_to_string(file) {
                let document_name = make_document(&content);
                if document == &document_name{
                    let num_paragraphs = count_paragraphs(document);
                    if document[0] == "" {table.push_str(format!("<td>{}</td><td>{}<td>",file, 0).as_str())}
                    else {table.push_str(format!("<td>{}</td><td>{}</td>", file, num_paragraphs).as_str());}
                }
            }
            
        }
        
        table.push_str("</tr>")
    }

    table.push_str("</tbody></table></body></html>");
    println!("{}", table);
}

fn count_words(para: Vec<String>) -> usize {
    let mut count = 0;
    for word in para {
        count += word.split_whitespace().count()
    }
    count
}

fn sort_count_words(docs: &mut Vec<(String, Vec<String>)>) {
    docs.sort_by(|a, b| count_words(b.1.clone()).cmp(&count_words(a.1.clone())));
}

fn count_html() {
    let file_paths = vec!["abc.txt", "bustle.txt", "fox.txt"];
    let mut documents: Vec<(String, Vec<String>)> = Vec::new(); // Tuple of (file name, paragraphs)

    for path in &file_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let document = make_document(&content);
            documents.push((path.to_string(), document)); // Push tuple of (file name, paragraphs)
        } else {
            println!("Error reading file: {}", path);
        }
    }

    sort_count_words(&mut documents);

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Manage Paragraph</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Manage Paragraph</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>File</th>
                        <th>Word Count</th>
                    </tr>
                </thead>
                <tbody>");

    for (file, document) in &documents {
        let num_words = count_words(document.clone());
        table.push_str(format!("<tr><td>{}</td><td>{}</td></tr>", file, num_words).as_str());
    }

    table.push_str("</tbody></table></body></html>");
    println!("{}", table);
}
pub fn get_client_reviews() -> Vec<ClientReview> {
    // Later on get client reviews from a csv or something and parse it.
    let review1 = ClientReview {
    name: String::from("Elon Gates"),
    company: String::from("Microsoft"),
    body: String::from("words more words and even more words we dont really care about"),
    };
    let review2 = ClientReview {
    name: String::from("Jeff Musk"),
    company: String::from("Apple"),
    body: String::from("words more words and even more words we dont really care about"),
    };
    let review3 = ClientReview {
    name: String::from("Bill Bezoz"),
    company: String::from("Amazon"),
    body: String::from("words more words and even more words we dont really care about"),
    };

    let reviews = vec![review1, review2, review3];

    return reviews;
    }

    /*
    let reviews = get_client_reviews();
    let mut reviews_html: Vec<Html> = Vec::new();

    for review in reviews {
    let review_html: Html = html! {
    <ClientReviewCard name={review.name} company={review.company} body={review.body} />};
    reviews_html.push(review_html);
    }
    */  
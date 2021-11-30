- Người dùng có thể tạo 1 câu hỏi và người khác có thể trả lời, mỗi người chỉ được trả lời 1 lần cho 1 câu hỏi. 1 câu hỏi bao gồm title và nội dung. 1 câu hỏi có thể được vote hoặc không (vote nhiều sẽ được xếp lên đầu). Yêu cầu list câu hỏi hiển thị title, content được cắt ngắn, số lượng người vote, số câu trả lời, thời gian tạo câu hỏi và người tạo câu hỏi
- 1 câu trả lời có thể được nhiều người khác hữu ích hoặc không (1 người chỉ có thể chọn 1 trong 2). 1 câu trả lời cũng có thể được donate bằng NEAR (1 người có thể donate nhiều lần, số tiền donate sẽ được chuyển vào ví của người trả lời). Câu trả lời càng được hữu ích nhiều sẽ được xếp lên đầu. Yêu cầu hiển thị bao gồm số câu trả lời, mỗi câu trả lời hiển thị nội dung, số lượt 'hữu ích', số tiền donate, thời gian trả lời và địa chỉ người trả lời, có thể xem lịch sử donate cho câu trả lời đó

Data struct

Question {
    question_id: String,
    title: String,
    content: String,
    total_vote: Number,
    total_answer: Number,
    created_time: Number (timestamp),
    creator_id: String (address)
}

Answer {
    answer_id: String,
    question_id: String,
    content: String,
    total_useful: Number,
    total_amount_donate: Number,
    created_time: Number (timestamp),
    creator_id: String (address)
}

DonateInfo {
    donate_info_id: String,
    answer_id: String,
    donate_creator_id: String,
    created_time: Number (timestamp),
    amount: Number
}

Map<String, Question> mapQuestion;
Map<String, Answer> mapAnswer;
Map<String, DonateInfo> mapDonateInfo;
Map<String, Set<String>> mapQuestionUserId; track user answer only one
Map<String, Set<String>> mapQuestionAnswer; quesion_id => list answer id
Map<String, Set<String>> mapAnswerDonation; answer_id => list donate id;


dto:
    QuestionCreateDto {
        title: String,
        content: String,
    }
    AnswerCreateDto {
        question_id: String,
        content: String,
    }
    DonationCreateDto {
        answer_id: String,
        amount: Number
    }
method:
    call
        + create_question(questionCreateDto: QuestionCreateDto) : Question
        + create_answer(answerCreateDto: AnswerCreateDto) : Answer
        + donate(donationCreateDto: DonationCreateDto) : DonateInfo

    view
        + get_list_question() : List<Question>
        + get_question_detail(question_id: String) : Question
        + get_list_answer_for_question(question_id: String) : List<Answer>
        + get_donate_history(answer_id: String) : List<DonateInfo>



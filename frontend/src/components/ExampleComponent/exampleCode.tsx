import getQuestionList from './questions'
import './example.css';
export default function ExampleCodeCollectionComponent({loadCode}: {loadCode: (s: string) => void}) {
    let questionList = getQuestionList().questions;
    return (
        <div>
            {
            questionList.map((questions, index) => {
                return (<div key={index} className="question-block">
                            <div className="question">{questions.question}</div>
                            <button onClick={() => loadCode(questions.code)}>Load Code</button>
                        </div>)
            })}
        </div>
    );
}

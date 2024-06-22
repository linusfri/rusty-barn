import React, { useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import Axios from 'axios';

document.addEventListener('DOMContentLoaded', () => {
    const modal = document.getElementById('modal');

    if (modal) {
        const root = createRoot(modal);
        root.render(<App />)
    }
});


type Faq = {
    question: string;
    answer: string;
}

export function App() {
    const [open, setOpen] = useState<Number[]>([]);
    const [faqs, setFaqs] = useState<Faq[]>([]);

    async function getFaqs() {
        const res = await Axios.get<Faq[]>('https://rustybarn.local:4430/about')

        setFaqs(res.data);
    }

    function toggleFaq(index: Number) {
        if (open.includes(index)) {
            setOpen(open.filter((i) => i !== index));
        } else {
            setOpen([...open, index]);
        }
    
    }

    useEffect(() => {
        getFaqs();
    }, [])

    const faqItems = faqs.map((faq, index) => {
        return (
            <div className='faq' key={index} onClick={() => toggleFaq(index)}>
                <h2 className='faq__question'>{faq.question}</h2>
                {open.includes(index) && <p className='answer'>{faq.answer}</p>}
            </div>
        )
    });

    return (
        <div className='modal'>
            {faqItems}
        </div>
    )
}
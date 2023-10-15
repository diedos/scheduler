import { PUBLIC_API_URL } from '$env/static/public';
import axios from 'axios';

export async function load() {
    const { data } = await axios
        .get(`${PUBLIC_API_URL}/tasks`, {
            headers: {
                'Content-Type': 'application/json'
            },
            data: { completed: false }
        })
        .catch((err) => {
            console.error(err);
            return { data: [] };
        });

    return {
        todoTasks: data
    };
}

export const actions = {
    create: async ({ request }) => {
        const data = await request.formData();
        axios
            .post(`${PUBLIC_API_URL}/tasks`, {
                title: data.get('title'),
                content: data.get('content'),
                deadlineAt: data.get('deadlineAt'),
                estimate: Number(data.get('estimate'))
            })
            .catch((err) => console.error(err));
    },
    complete: async ({ request }) => {
        const formData = await request.formData();
        const { data } = await axios
            .post(`${PUBLIC_API_URL}/tasks/${formData.get('id')}/complete`)
            .catch((err) => {
                console.error(err);
                return { data: { nextTask: undefined } };
            });
        return {
            nextTask: data
        };
    },
    delete: async ({ request }) => {
        const data = await request.formData();
        axios
            .delete(`${PUBLIC_API_URL}/tasks/` + data.get('id'))
            .catch((err) => console.error(err));
    }
};

import axios from 'axios';

export async function load() {
    const { data } = await axios.get('http://127.0.0.1:3000/tasks').catch((err) => {
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
            .post('http://127.0.0.1:3000/tasks', {
                title: data.get('title'),
                content: data.get('content'),
                deadlineAt: data.get('deadlineAt'),
                estimate: data.get('estimate')
            })
            .catch((err) => console.error(err));
    }
};

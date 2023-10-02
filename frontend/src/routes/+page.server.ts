import axios from 'axios';

export async function load() {
    const { data } = await axios
        .get('http://127.0.0.1:3000/tasks', {
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
            .post('http://127.0.0.1:3000/tasks', {
                title: data.get('title'),
                content: data.get('content'),
                deadlineAt: data.get('deadlineAt'),
                estimate: Number(data.get('estimate'))
            })
            .catch((err) => console.error(err));
    },
    complete: async ({ request }) => {
        const data = await request.formData();
        axios
            .post(`http://127.0.0.1:3000/tasks/${data.get('id')}/complete`)
            .catch((err) => console.error(err));
    },
    delete: async ({ request }) => {
        const data = await request.formData();
        axios
            .delete('http://127.0.0.1:3000/tasks/' + data.get('id'))
            .catch((err) => console.error(err));
    }
};

import axios from 'axios';

export async function load({ params }) {
    const id = await params.id;
    const { data } = await axios.get(`http://127.0.0.1:3000/tasks/${id}`).catch((err) => {
        console.error(err);
        return {};
    });

    return data;
}

import { PUBLIC_API_URL } from '$env/static/public';
import axios from 'axios';

export async function load({ params }) {
    const id = await params.id;
    const { data } = await axios.get(`${PUBLIC_API_URL}/tasks/${id}`).catch((err) => {
        console.error(err);
        return {};
    });

    return data;
}

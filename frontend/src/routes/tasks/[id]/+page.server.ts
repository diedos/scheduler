import { API_URL } from '$env/static/private';
import axios from 'axios';

export async function load({ params }) {
    const id = await params.id;
    const { data } = await axios.get(`${API_URL}/tasks/${id}`).catch((err) => {
        console.error(err);
        return {};
    });

    return data;
}

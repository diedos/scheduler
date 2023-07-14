import { writable, type Writable } from 'svelte/store';

type TodoTask = {
    id: number;
    created_at: string;
    modified_at: string;
    title: string;
    content: string;
    completed: boolean;
};

export const todoTasks: Writable<TodoTask[]> = writable([]);

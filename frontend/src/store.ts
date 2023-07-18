import { writable, type Writable } from 'svelte/store';

type TodoTask = {
    id: number;
    createdAt: string;
    updatedAt: string;
    completedAt: string;
    deadlineAt: string;
    title: string;
    content: string;
    completed: boolean;
};

export const todoTasks: Writable<TodoTask[]> = writable([]);

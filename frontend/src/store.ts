import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

export type TodoTask = {
    id: number;
    createdAt: string;
    updatedAt: string;
    completedAt: string;
    deadlineAt: string;
    title: string;
    content: string;
};

const emptyTask = {
    id: 0,
    createdAt: '',
    updatedAt: '',
    completedAt: '',
    deadlineAt: '',
    title: 'No task selected',
    content: ''
};

export const todoTasks: Writable<TodoTask[]> = writable([]);

export const currentTask: Writable<TodoTask | undefined> = writable(undefined);

if (browser) {
    const storedTask = localStorage.getItem('currentTask');

    currentTask.set(storedTask && storedTask.length > 0 ? JSON.parse(storedTask) : emptyTask);
    currentTask.subscribe((task) => {
        localStorage.setItem('currentTask', task ? JSON.stringify(task) : '');
    });
}

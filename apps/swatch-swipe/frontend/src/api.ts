import axios from 'axios';
import { Palette, SavedPalette } from './types';

const api = axios.create({ baseURL: '/api' });

export async function generatePalettes(premise: string): Promise<Palette[]> {
  const { data } = await api.post('/generate', { premise });
  return data.palettes;
}

export async function getSavedPalettes(): Promise<SavedPalette[]> {
  const { data } = await api.get('/palettes');
  return data;
}

export async function savePalette(palette: Palette, premise: string): Promise<SavedPalette> {
  const { data } = await api.post('/palettes/save', { ...palette, premise });
  return data;
}

export async function deletePalette(id: string): Promise<void> {
  await api.delete(`/palettes/${id}`);
}

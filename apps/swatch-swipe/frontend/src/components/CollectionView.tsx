import { SavedPalette } from '../types';
import { PaletteCard } from './PaletteCard';
import { deletePalette } from '../api';

interface Props {
  palettes: SavedPalette[];
  onDelete: (id: string) => void;
}

export function CollectionView({ palettes, onDelete }: Props) {
  const handleDelete = async (id: string) => {
    try {
      await deletePalette(id);
      onDelete(id);
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  };

  if (palettes.length === 0) {
    return (
      <div className="flex flex-col items-center gap-4 pt-16 text-center">
        <h2 className="text-xl font-bold">No saved palettes yet</h2>
        <p className="text-zinc-400">Generate some palettes and swipe right to save your favorites.</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6">
      <h2 className="text-xl font-bold">Your Collection</h2>
      {palettes.map(palette => (
        <div key={palette.id} className="relative group">
          <PaletteCard palette={palette} compact />
          <button
            onClick={() => handleDelete(palette.id)}
            className="absolute top-3 right-3 opacity-0 group-hover:opacity-100 transition-opacity bg-red-600/80 hover:bg-red-600 text-white text-xs px-3 py-1 rounded-lg"
          >
            Remove
          </button>
          <p className="mt-1 text-zinc-600 text-xs px-1">Premise: "{palette.premise}"</p>
        </div>
      ))}
    </div>
  );
}

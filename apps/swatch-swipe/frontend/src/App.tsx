import { useState } from 'react';
import { GeneratorView } from './components/GeneratorView';
import { CollectionView } from './components/CollectionView';
import { SavedPalette } from './types';

function App() {
  const [view, setView] = useState<'generator' | 'collection'>('generator');
  const [savedPalettes, setSavedPalettes] = useState<SavedPalette[]>([]);

  const handleSave = (palette: SavedPalette) => {
    setSavedPalettes(prev => [palette, ...prev]);
  };

  const handleDelete = (id: string) => {
    setSavedPalettes(prev => prev.filter(p => p.id !== id));
  };

  return (
    <div className="min-h-screen bg-zinc-950 text-white">
      <nav className="border-b border-zinc-800 px-6 py-4 flex items-center justify-between">
        <h1 className="text-xl font-bold tracking-tight">Swatch Swipe</h1>
        <div className="flex gap-2">
          <button
            onClick={() => setView('generator')}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
              view === 'generator' ? 'bg-white text-black' : 'bg-zinc-800 text-zinc-400 hover:text-white'
            }`}
          >
            Generate
          </button>
          <button
            onClick={() => setView('collection')}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
              view === 'collection' ? 'bg-white text-black' : 'bg-zinc-800 text-zinc-400 hover:text-white'
            }`}
          >
            Collection {savedPalettes.length > 0 && `(${savedPalettes.length})`}
          </button>
        </div>
      </nav>
      <main className="max-w-2xl mx-auto px-4 py-8">
        {view === 'generator' ? (
          <GeneratorView onSave={handleSave} />
        ) : (
          <CollectionView palettes={savedPalettes} onDelete={handleDelete} />
        )}
      </main>
    </div>
  );
}

export default App;

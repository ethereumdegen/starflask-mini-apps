import { useState } from 'react';
import { GeneratorView } from './components/GeneratorView';
import { CollectionView } from './components/CollectionView';
import type { SavedPalette } from './types';

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
      <footer className="border-t border-zinc-800 mt-16 py-8 px-6 text-center text-zinc-500 text-sm leading-relaxed">
        <p>
          This app's agentic loop is powered by{' '}
          <a href="https://starflask.com" className="text-zinc-300 hover:text-white underline underline-offset-2" target="_blank" rel="noopener noreferrer">starflask.com</a>
          , making the code incredibly lightweight and simple.
        </p>
        <p className="mt-2">
          Built entirely by Claude Code. Source available on{' '}
          <a href="https://github.com/ethereumdegen/starflask-mini-apps" className="text-zinc-300 hover:text-white underline underline-offset-2" target="_blank" rel="noopener noreferrer">GitHub</a>.
        </p>
        <div className="mt-6 max-w-2xl mx-auto text-left">
          <p className="text-zinc-500 text-xs mb-2">This webapp was built with a query to Claude Code as follows:</p>
          <pre className="bg-zinc-900 border border-zinc-800 rounded-lg p-4 text-xs text-zinc-400 whitespace-pre-wrap overflow-x-auto">
{`Build a webapp using vite and react with a rust backend server that is like Tinder but for color palettes. It will ask the user for a query and use a starflask.com ai agent to process the query, return the result, and render the color palette for the user on this page. You will need to use the starflask rust crate and also build a rust script to seed the initial agent config (that you will build) in order to agentically process these queries. Research starflask crate and starflask.com and build this webapp in /palette-selector-app`}
          </pre>
        </div>
      </footer>
    </div>
  );
}

export default App;

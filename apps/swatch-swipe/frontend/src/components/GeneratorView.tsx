import { useState } from 'react';
import { generatePalettes, savePalette } from '../api';
import { Palette, SavedPalette } from '../types';
import { PaletteCard } from './PaletteCard';
import { motion, AnimatePresence } from 'framer-motion';

interface Props {
  onSave: (palette: SavedPalette) => void;
}

export function GeneratorView({ onSave }: Props) {
  const [premise, setPremise] = useState('');
  const [queue, setQueue] = useState<Palette[]>([]);
  const [loading, setLoading] = useState(false);
  const [currentPremise, setCurrentPremise] = useState('');
  const [direction, setDirection] = useState<'left' | 'right' | null>(null);

  const handleGenerate = async () => {
    if (!premise.trim()) return;
    setLoading(true);
    try {
      const palettes = await generatePalettes(premise.trim());
      setQueue(palettes);
      setCurrentPremise(premise.trim());
    } catch (err) {
      console.error('Failed to generate:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSwipe = async (liked: boolean) => {
    const current = queue[0];
    if (!current) return;

    setDirection(liked ? 'right' : 'left');

    if (liked) {
      try {
        const saved = await savePalette(current, currentPremise);
        onSave(saved);
      } catch (err) {
        console.error('Failed to save:', err);
      }
    }

    setTimeout(() => {
      setQueue(prev => prev.slice(1));
      setDirection(null);
    }, 200);
  };

  if (queue.length === 0) {
    return (
      <div className="flex flex-col items-center gap-6 pt-16">
        <div className="text-center">
          <h2 className="text-2xl font-bold mb-2">What's your vibe?</h2>
          <p className="text-zinc-400">Describe a design theme and we'll generate color palettes for you.</p>
        </div>
        <div className="w-full max-w-md flex flex-col gap-3">
          <input
            type="text"
            value={premise}
            onChange={e => setPremise(e.target.value)}
            onKeyDown={e => e.key === 'Enter' && handleGenerate()}
            placeholder="e.g. cyberpunk dashboard, cozy coffee shop, minimal SaaS..."
            className="w-full px-4 py-3 bg-zinc-900 border border-zinc-700 rounded-xl text-white placeholder-zinc-500 focus:outline-none focus:border-zinc-500 transition-colors"
            disabled={loading}
          />
          <button
            onClick={handleGenerate}
            disabled={loading || !premise.trim()}
            className="px-6 py-3 bg-white text-black font-semibold rounded-xl hover:bg-zinc-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Generating...' : 'Generate Palettes'}
          </button>
        </div>
        {loading && (
          <div className="flex items-center gap-2 text-zinc-500">
            <div className="w-4 h-4 border-2 border-zinc-600 border-t-white rounded-full animate-spin" />
            <span>Crafting your palettes...</span>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center gap-6">
      <p className="text-zinc-400 text-sm">
        {queue.length} palette{queue.length !== 1 ? 's' : ''} remaining · <span className="text-zinc-500">"{currentPremise}"</span>
      </p>
      <div className="relative w-full" style={{ minHeight: 420 }}>
        <AnimatePresence mode="popLayout">
          <motion.div
            key={queue[0].name}
            initial={{ scale: 0.95, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{
              x: direction === 'right' ? 300 : direction === 'left' ? -300 : 0,
              opacity: 0,
              transition: { duration: 0.2 }
            }}
          >
            <PaletteCard palette={queue[0]} />
          </motion.div>
        </AnimatePresence>
      </div>
      <div className="flex gap-4">
        <button
          onClick={() => handleSwipe(false)}
          className="px-8 py-3 bg-zinc-800 text-zinc-400 font-semibold rounded-xl hover:bg-zinc-700 hover:text-white transition-colors"
        >
          Skip
        </button>
        <button
          onClick={() => handleSwipe(true)}
          className="px-8 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-500 transition-colors"
        >
          Save
        </button>
      </div>
      <button
        onClick={() => { setQueue([]); setPremise(''); }}
        className="text-zinc-500 text-sm hover:text-zinc-300 transition-colors"
      >
        Start over
      </button>
    </div>
  );
}

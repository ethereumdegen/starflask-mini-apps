import { Palette } from '../types';

interface Props {
  palette: Palette;
  compact?: boolean;
}

export function PaletteCard({ palette, compact }: Props) {
  return (
    <div className="bg-zinc-900 rounded-2xl overflow-hidden border border-zinc-800">
      <div className="p-4 pb-2">
        <h3 className="text-lg font-bold">{palette.name}</h3>
        <p className="text-zinc-400 text-sm">{palette.mood}</p>
      </div>
      <div>
        {palette.colors.map((color, i) => {
          const isLight = isLightColor(color.hex);
          return (
            <div
              key={i}
              className={`flex items-center justify-between px-4 ${compact ? 'py-3' : 'py-5'}`}
              style={{ backgroundColor: color.hex }}
            >
              <span className={`font-medium text-sm ${isLight ? 'text-black' : 'text-white'}`}>
                {color.name}
              </span>
              <div className="flex items-center gap-3">
                <span className={`text-xs font-mono ${isLight ? 'text-black/60' : 'text-white/60'}`}>
                  {color.hex}
                </span>
                <span className={`text-xs px-2 py-0.5 rounded-full ${
                  isLight ? 'bg-black/10 text-black/70' : 'bg-white/10 text-white/70'
                }`}>
                  {color.role}
                </span>
              </div>
            </div>
          );
        })}
      </div>
      {!compact && (
        <div className="p-4 pt-2 border-t border-zinc-800">
          <p className="text-zinc-500 text-sm">{palette.use_case}</p>
        </div>
      )}
    </div>
  );
}

function isLightColor(hex: string): boolean {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  return luminance > 0.5;
}

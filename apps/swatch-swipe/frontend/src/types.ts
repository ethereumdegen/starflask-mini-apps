export interface PaletteColor {
  hex: string;
  name: string;
  role: string;
}

export interface Palette {
  name: string;
  mood: string;
  colors: PaletteColor[];
  use_case: string;
}

export interface SavedPalette extends Palette {
  id: string;
  premise: string;
}

export interface CountsByNoun {
  noun: string;
  counts: number;
}

export interface CountsOfNounByYear {
  year: number;
  nouns: CountsByNoun[];
}

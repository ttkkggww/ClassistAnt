import React from 'react';
import styles from './Grid.module.css'; // スタイリングのためのCSSファイル
import {rgbToHex} from '../../modules/color';

interface GridProps {
    data: string[][];
    pheromone_256: number[][];
}

const GridComponent: React.FC<GridProps> = ({ data ,pheromone_256}) => {
  return (
    <div className={styles['grid-container']} style={{}}>
      {data.map((row, rowIndex) => (
          row.map((cell, columnIndex) => 
          {
            let ph = pheromone_256[rowIndex][columnIndex];
            console.log(ph)
          return (
            <div className={styles['grid-cell']} style={{backgroundColor: rgbToHex(255,255-ph,255-ph)}}>
              {cell}
            </div>
          )}
          )
      ))}
    </div>
  );
};

export default GridComponent;

import React from 'react';
import styles from './Grid.module.css'; // スタイリングのためのCSSファイル
import {rgbToHex} from '../../modules/color';

interface GridProps {
    data: string[][];
    pheromone_256: number[][];
    messages: string[];
}

const GridComponent: React.FC<GridProps> = ({ data ,pheromone_256,messages}) => {
  return (
    <div>
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
   <div>
      {messages.map((str, index) => (
        <React.Fragment key={index}>
          {str}
          {index < messages.length - 1 && <br />} {/* 最後の要素以外に改行を挿入 */}
        </React.Fragment>
      ))}
    </div>
    </div>
  );
};

export default GridComponent;

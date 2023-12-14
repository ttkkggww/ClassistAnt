import React from 'react';
import styles from './Grid.module.css'; // スタイリングのためのCSSファイル

interface GridProps {
    data: string[][];
}

const GridComponent: React.FC<GridProps> = ({ data }) => {
  return (
    <div className={styles['grid-container']}>
      {data.map((row, rowIndex) => (
        <div key={rowIndex} className={styles['grid-row']}>
          {row.map((cell, columnIndex) => (
            <div key={columnIndex} className={styles['grid-cell']}>
              {cell}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
};

export default GridComponent;

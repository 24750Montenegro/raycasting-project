"""Convierte un video en la hoja de cuadros que carga el motor.

El juego no decodifica video: las texturas viven en RAM como pixeles y el
raycaster las muestrea por columna, asi que un video animado se guarda igual que
cualquier otra textura, con sus cuadros uno al lado del otro en una sola imagen.
Este script hace esa conversion una vez, fuera del juego.

    python tools/video_to_sheet.py assets/dog-knife.mp4 assets/dog-knife.png

Imprime cuantos cuadros salieron y a que velocidad iba el original, que es lo
que hay que copiar a la entrada de config::WALL_TEXTURES.

Necesita opencv-python (pip install opencv-python).
"""

import argparse
import sys

import cv2
import numpy as np


def frames_of(path):
    capture = cv2.VideoCapture(path)
    if not capture.isOpened():
        sys.exit(f"no se pudo abrir {path}")

    fps = capture.get(cv2.CAP_PROP_FPS)
    frames = []
    while True:
        ok, frame = capture.read()
        if not ok:
            break
        frames.append(frame)
    capture.release()

    if not frames:
        sys.exit(f"{path} no tiene cuadros")
    return frames, fps


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("video")
    parser.add_argument("sheet")
    # el motor reescala de nuevo a config::TEXTURE_MAX_SIZE al cargar; esto solo
    # evita guardar en el repo mas resolucion de la que se va a ver
    parser.add_argument("--size", type=int, default=256,
                        help="lado maximo de cada cuadro (por defecto 256)")
    args = parser.parse_args()

    frames, fps = frames_of(args.video)

    scale = args.size / max(frames[0].shape[:2])
    if scale < 1.0:
        width = max(1, round(frames[0].shape[1] * scale))
        height = max(1, round(frames[0].shape[0] * scale))
        frames = [cv2.resize(f, (width, height), interpolation=cv2.INTER_AREA)
                  for f in frames]

    cv2.imwrite(args.sheet, np.hstack(frames))
    height, width = frames[0].shape[:2]
    print(f"{args.sheet}: {len(frames)} cuadros de {width}x{height} a {fps:g} fps")


if __name__ == "__main__":
    main()

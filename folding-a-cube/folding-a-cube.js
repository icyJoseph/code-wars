/**
 * Solution derived from my AoC Day 22 solver:
 * - https://adventofcode.com/2022/day/22
 * - https://github.com/icyJoseph/advent-of-code/blob/master/2022/deno/22.ts
 *
 * It builds a cube, or fails at it, in 6 steps, doing DFS.
 *
 */

const createCube = () => {
  const faces = {};

  const rotateFwd = () => {
    const current = { ...faces };

    faces.top = current.back;
    faces.front = current.top;
    faces.bottom = current.front;
    faces.back = current.bottom;

    faces.right = current.right;
    faces.left = current.left;
  };

  const rotateBwd = () => {
    const current = { ...faces };

    faces.top = current.front;
    faces.back = current.top;
    faces.bottom = current.back;
    faces.front = current.bottom;

    faces.right = current.right;
    faces.left = current.left;
  };

  const rotateRight = () => {
    const current = { ...faces };

    faces.top = current.left;
    faces.right = current.top;
    faces.bottom = current.right;
    faces.left = current.bottom;

    faces.front = current.front;
    faces.back = current.back;
  };

  const rotateLeft = () => {
    const current = { ...faces };

    faces.top = current.right;
    faces.left = current.top;
    faces.bottom = current.left;
    faces.right = current.bottom;

    faces.front = current.front;
    faces.back = current.back;
  };

  return {
    get faces() {
      return { ...faces };
    },
    setFace(side, value) {
      faces[side] = value;
    },
    rotateFwd,
    rotateBwd,
    rotateLeft,
    rotateRight,
  };
};

// Starting at any given face, roll into the
// adjacent faces, taking each as the new bottom of the cube
const buildCube = (cube, face, seen = new Set()) => {
  cube.setFace("bottom", face.value);
  seen.add(face);

  if (face.top && !seen.has(face.top)) {
    cube.rotateBwd();
    buildCube(cube, face.top, seen);
    cube.rotateFwd();
  }

  if (face.bottom && !seen.has(face.bottom)) {
    cube.rotateFwd();
    buildCube(cube, face.bottom, seen);
    cube.rotateBwd();
  }

  if (face.left && !seen.has(face.left)) {
    cube.rotateLeft();
    buildCube(cube, face.left, seen);
    cube.rotateRight();
  }

  if (face.right && !seen.has(face.right)) {
    cube.rotateRight();
    buildCube(cube, face.right, seen);
    cube.rotateLeft();
  }

  return cube;
};

const dirs = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
];

const calcAdj = (faces) => {
  const adj = Object.keys(faces).map((key) => ({
    key,
    value: faces[key],
  }));

  adj.forEach((face) => {
    const { x, y } = face.value;

    dirs.forEach(([dy, dx]) => {
      const otherKey = `${y + dy}.${x + dx}`;
      const other = faces[otherKey];

      if (!other) return;

      const otherAdj = adj.find((ot) => ot.value === other);

      if (!otherAdj) return;

      if (dx === -1) {
        // left
        face.left = otherAdj;
        return;
      }

      if (dx === 1) {
        // left
        face.right = otherAdj;
        return;
      }

      if (dy === -1) {
        // left
        face.top = otherAdj;
        return;
      }

      if (dy === 1) {
        // left
        face.bottom = otherAdj;
        return;
      }
    });
  });

  return adj;
};

// return true or false
const fold_cube = function (input_list) {
  const cube = createCube();

  const lookUp = input_list
    .slice(0)
    .sort((a, b) => a - b)
    .map((n) => n - 1)
    .map((n) => ({
      x: n % 5,
      y: Math.floor(n / 5),
    }))
    .reduce((prev, { x, y }) => {
      prev[`${y}.${x}`] = { x, y };
      return prev;
    }, {});

  const faceAdj = calcAdj(lookUp);

  buildCube(cube, faceAdj[0]);

  return Object.values(cube.faces).filter(Boolean).length === 6;
};

// console.log(fold_cube([24, 20, 14, 19, 18, 9]));
// console.log(fold_cube([1, 7, 6, 17, 12, 16]));

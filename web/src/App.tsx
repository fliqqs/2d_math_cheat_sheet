import { Navbar } from '@fliqqs/portfolio-scaffold'
import WasmDemo from './components/WasmDemo'
import './App.css'

function App() {

  return (
    <div className="app">
      <Navbar
        title="2D Math Cheat Sheet"
        siteName="fliqqs"
        homeUrl="https://fliqqs.github.io"
        links={[]}
      />
      <main className="container">
        <article className="blog-post">
          <header className="post-header">
            <h1>2D Math Cheat Sheet: From Formulas to Action</h1>
            <p className="post-meta">Interactive guide to game math fundamentals</p>
          </header>

          <section className="intro">
            <p>
              I've always found math formulas on Wikipedia and textbooks hard to internalize. You see something like:
            </p>
            <pre><code>atan2(y, x) = arctan(y/x)</code></pre>
            <p>
              ...and your eyes glaze over. What does that <em>actually do</em>? When would I use it?
            </p>
            <p>
              So I built an interactive 2D world where you can <em>see</em> these formulas in action.
              Move the mouse, watch the numbers change, and suddenly the abstract becomes concrete.
            </p>
          </section>

          <section className="demo-section">
            <h2># Interactive Demo</h2>
            <p className="demo-instructions">Use <span className="key">WASD</span> to move, point with mouse</p>
            <WasmDemo />
          </section>

          <section className="concept">
            <h2># ATAN2: Finding the Angle to a Target</h2>

            <h3>## The Formula</h3>
            <pre><code>angle = atan2(dy, dx)</code></pre>
            <p>Where:</p>
            <ul>
              <li><code>dx</code> = target.x - origin.x</li>
              <li><code>dy</code> = target.y - origin.y</li>
            </ul>

            <h3>## What the textbook says</h3>
            <blockquote>
              "Returns the angle in radians between the positive x-axis and the point (x, y)."
            </blockquote>

            <h3>## What that actually means</h3>
            <p>
              You have a character. There's an enemy. You want to know what angle to rotate your
              character to face the enemy. That's atan2.
            </p>

            <h3>## Step by step</h3>
            <p>Let's say your player is at <code>(2, 1)</code> and the mouse is at <code>(5, 4)</code>:</p>
            <pre><code>{`Step 1: Calculate the difference
  dx = 5 - 2 = 3
  dy = 4 - 1 = 3

Step 2: Apply atan2
  angle = atan2(3, 3)
  angle = 0.785 radians

Step 3: Convert to degrees (optional)
  degrees = 0.785 * (180 / PI)
  degrees = 45°`}</code></pre>
            <p>The result: you need to rotate 45 degrees from the positive x-axis to face the target.</p>

            <h3>## Why atan2 instead of atan?</h3>
            <p>
              Regular <code>atan(y/x)</code> breaks when x is zero (division by zero) and can't tell
              the difference between opposite directions. <code>atan2</code> handles all four quadrants correctly:
            </p>
            <table>
              <thead>
                <tr><th>dx</th><th>dy</th><th>atan2 result</th></tr>
              </thead>
              <tbody>
                <tr><td>+</td><td>+</td><td>0° to 90°</td></tr>
                <tr><td>-</td><td>+</td><td>90° to 180°</td></tr>
                <tr><td>-</td><td>-</td><td>-180° to -90°</td></tr>
                <tr><td>+</td><td>-</td><td>-90° to 0°</td></tr>
              </tbody>
            </table>

            <h3>## Code</h3>
            <pre><code>{`let dx = target.x - origin.x;
let dy = target.y - origin.y;
let angle = dy.atan2(dx);  // Note: y comes first!`}</code></pre>
          </section>

          <section className="concept">
            <h2># 2D Rotation: Spinning Points Around</h2>

            <h3>## The Formula</h3>
            <pre><code>{`x' = x * cos(angle) - y * sin(angle)
y' = x * sin(angle) + y * cos(angle)`}</code></pre>
            <p>Or in matrix form:</p>
            <pre><code>{`[x']   [cos(a)  -sin(a)] [x]
[y'] = [sin(a)   cos(a)] [y]`}</code></pre>

            <h3>## What the textbook says</h3>
            <blockquote>
              "A rotation matrix rotates points in the xy-plane counterclockwise through an angle about the origin."
            </blockquote>

            <h3>## What that actually means</h3>
            <p>
              You have a spaceship pointing right. The player presses "rotate left." You need to rotate
              every point of that spaceship by some angle. This formula does that.
            </p>

            <h3>## Step by step</h3>
            <p>Let's rotate the point <code>(1.5, 0)</code> by 90 degrees:</p>
            <pre><code>{`Step 1: Convert angle to radians
  angle = 90° * (PI / 180) = 1.571 radians

Step 2: Calculate cos and sin
  cos(90°) = 0
  sin(90°) = 1

Step 3: Apply the formula
  x' = 1.5 * 0 - 0 * 1 = 0
  y' = 1.5 * 1 + 0 * 0 = 1.5

Result: (1.5, 0) -> (0, 1.5)`}</code></pre>
            <p>The point moved from "pointing right" to "pointing up" - exactly 90 degrees counterclockwise!</p>

            <h3>## Rotating around a different center</h3>
            <p>The formula above rotates around the origin (0, 0). To rotate around a different point:</p>
            <pre><code>{`// 1. Translate to origin
let local_x = point.x - center.x;
let local_y = point.y - center.y;

// 2. Rotate
let rotated_x = local_x * cos(angle) - local_y * sin(angle);
let rotated_y = local_x * sin(angle) + local_y * cos(angle);

// 3. Translate back
let final_x = rotated_x + center.x;
let final_y = rotated_y + center.y;`}</code></pre>

            <h3>## Code</h3>
            <pre><code>{`let cos_a = angle.cos();
let sin_a = angle.sin();

let rotated_x = original.x * cos_a - original.y * sin_a;
let rotated_y = original.x * sin_a + original.y * cos_a;`}</code></pre>
          </section>

          <section className="concept">
            <h2># Dot Product: How Similar Are Two Directions?</h2>

            <h3>## The Formula</h3>
            <pre><code>A · B = Ax * Bx + Ay * By</code></pre>
            <p>Or equivalently:</p>
            <pre><code>A · B = |A| * |B| * cos(angle_between)</code></pre>

            <h3>## What the textbook says</h3>
            <blockquote>
              "The dot product is the sum of the products of the corresponding entries of two sequences of numbers."
            </blockquote>

            <h3>## What that actually means</h3>
            <p>The dot product tells you how much two vectors "agree" with each other:</p>
            <ul>
              <li><span className="positive">Positive</span>: They're pointing roughly the same direction</li>
              <li><span className="neutral">Zero</span>: They're perpendicular (90 degrees apart)</li>
              <li><span className="negative">Negative</span>: They're pointing roughly opposite directions</li>
            </ul>

            <h3>## Use cases</h3>
            <ol>
              <li><strong>Is the enemy in front of me?</strong> Dot product of "forward" and "to_enemy" &gt; 0? They're in front.</li>
              <li><strong>Lighting:</strong> How directly is a surface facing a light?</li>
              <li><strong>Field of view:</strong> Is something within my vision cone?</li>
            </ol>

            <h3>## Step by step</h3>
            <p>Vector A = <code>(2, 0)</code> (pointing right)<br/>
            Vector B = <code>(1, 1)</code> (pointing up-right, 45 degrees)</p>
            <pre><code>{`Step 1: Multiply corresponding components
  Ax * Bx = 2 * 1 = 2
  Ay * By = 0 * 1 = 0

Step 2: Sum them
  dot = 2 + 0 = 2

Result: Positive, so they're pointing in similar directions`}</code></pre>

            <h3>## With unit vectors</h3>
            <p>If both vectors are normalized (length = 1), the dot product directly gives you <code>cos(angle)</code>:</p>
            <pre><code>{`A_unit = (1, 0)
B_unit = (0.707, 0.707)  // normalized (1,1)

dot = 1 * 0.707 + 0 * 0.707 = 0.707

angle = acos(0.707) = 45 degrees`}</code></pre>

            <h3>## Code</h3>
            <pre><code>{`// Basic dot product
let dot = a.x * b.x + a.y * b.y;

// Or using built-in method
let dot = vec_a.dot(vec_b);

// Check if in front
if forward.dot(to_target) > 0.0 {
    println!("Target is in front!");
}`}</code></pre>
          </section>

          <section className="concept">
            <h2># Lerp: Smooth Movement Between Points</h2>

            <h3>## The Formula</h3>
            <pre><code>result = A + t * (B - A)</code></pre>
            <p>Or equivalently:</p>
            <pre><code>result = A * (1 - t) + B * t</code></pre>
            <p>Where <code>t</code> ranges from 0 to 1.</p>

            <h3>## What the textbook says</h3>
            <blockquote>
              "Linear interpolation is a method of curve fitting using linear polynomials to construct new data points within the range of a discrete set of known data points."
            </blockquote>

            <h3>## What that actually means</h3>
            <p>You want to smoothly move something from point A to point B. Lerp gives you any point along that line:</p>
            <ul>
              <li><code>t = 0</code> → You're at A</li>
              <li><code>t = 0.5</code> → You're halfway between A and B</li>
              <li><code>t = 1</code> → You're at B</li>
            </ul>

            <h3>## Use cases</h3>
            <ol>
              <li><strong>Animation:</strong> Smoothly move a character from start to end position</li>
              <li><strong>Color blending:</strong> Fade from red to blue</li>
              <li><strong>Camera follow:</strong> Smooth camera movement instead of snapping</li>
              <li><strong>Health bars:</strong> Smooth transitions when taking damage</li>
            </ol>

            <h3>## Step by step</h3>
            <p>Point A = <code>(-2.5, -1)</code><br/>
            Point B = <code>(2.5, 1)</code><br/>
            t = 0.3 (30% of the way)</p>
            <pre><code>{`Step 1: Calculate the difference (B - A)
  diff_x = 2.5 - (-2.5) = 5
  diff_y = 1 - (-1) = 2

Step 2: Scale by t
  scaled_x = 0.3 * 5 = 1.5
  scaled_y = 0.3 * 2 = 0.6

Step 3: Add to A
  result_x = -2.5 + 1.5 = -1.0
  result_y = -1 + 0.6 = -0.4

Result: (-1.0, -0.4)`}</code></pre>

            <h3>## Smooth movement over time</h3>
            <pre><code>{`// Move 20% of remaining distance each frame (ease-out)
position = lerp(position, target, 0.2);

// Move at constant speed
t += speed * delta_time;
position = lerp(start, end, t.clamp(0.0, 1.0));`}</code></pre>

            <h3>## Code</h3>
            <pre><code>{`fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

// For vectors
fn lerp_vec(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t),
    )
}

// Or simply
let result = a + t * (b - a);`}</code></pre>
          </section>

          <section className="concept">
            <h2># Point at Angle + Distance</h2>

            <h3>## The Formula</h3>
            <pre><code>{`x = origin.x + cos(angle) * distance
y = origin.y + sin(angle) * distance`}</code></pre>

            <h3>## What the textbook says</h3>
            <blockquote>
              "Converting from polar coordinates (r, θ) to Cartesian coordinates (x, y)."
            </blockquote>

            <h3>## What that actually means</h3>
            <p>
              You're at position (x, y). You want to spawn a bullet 3 units in front of you.
              Or place a health bar 2 units above your character. Or spawn enemies in a circle
              around a boss. This formula does all of that.
            </p>

            <h3>## Use cases</h3>
            <ol>
              <li><strong>Projectile spawning:</strong> Fire a bullet from the gun barrel, not the player center</li>
              <li><strong>Orbit patterns:</strong> Place objects in a circle around a point</li>
              <li><strong>Offset spawning:</strong> Spawn particles at an offset from an explosion</li>
              <li><strong>Relative positioning:</strong> Place UI elements relative to a character</li>
            </ol>

            <h3>## Step by step</h3>
            <p>Your player is at <code>(0, -14)</code>, facing 60 degrees. Spawn a projectile 3 units away:</p>
            <pre><code>{`Step 1: Convert angle to radians
  angle = 60° * (PI / 180) = 1.047 radians

Step 2: Calculate cos and sin
  cos(60°) = 0.5
  sin(60°) = 0.866

Step 3: Apply the formula
  x = 0 + 0.5 * 3 = 1.5
  y = -14 + 0.866 * 3 = -11.4

Result: The point is at (1.5, -11.4)`}</code></pre>

            <h3>## Adding an offset angle</h3>
            <p>Spawn projectiles relative to where you're facing (e.g., spread shot):</p>
            <pre><code>{`base_angle = atan2(target.y - origin.y, target.x - origin.x)

// Left projectile (-45 degrees)
left_angle = base_angle + (-45° in radians)
left_x = origin.x + cos(left_angle) * distance
left_y = origin.y + sin(left_angle) * distance

// Right projectile (+45 degrees)
right_angle = base_angle + (45° in radians)
right_x = origin.x + cos(right_angle) * distance
right_y = origin.y + sin(right_angle) * distance`}</code></pre>

            <h3>## Code</h3>
            <pre><code>{`// Basic: point at angle and distance from origin
fn point_at_angle(origin: Vec2, angle: f32, distance: f32) -> Vec2 {
    Vec2::new(
        origin.x + angle.cos() * distance,
        origin.y + angle.sin() * distance,
    )
}

// With offset from a base direction
fn point_at_offset(
    origin: Vec2,
    base_angle: f32,
    offset_deg: f32,
    distance: f32
) -> Vec2 {
    let final_angle = base_angle + offset_deg.to_radians();
    Vec2::new(
        origin.x + final_angle.cos() * distance,
        origin.y + final_angle.sin() * distance,
    )
}`}</code></pre>
          </section>

          <section className="concept">
            <h2># Bonus: Unit Vectors (Normalizing)</h2>

            <h3>## The Formula</h3>
            <pre><code>{`unit = vector / |vector|

where |vector| = sqrt(x² + y²)`}</code></pre>

            <h3>## Step by step</h3>
            <p>Vector = <code>(3, 4)</code></p>
            <pre><code>{`Step 1: Calculate length
  length = sqrt(3² + 4²)
  length = sqrt(9 + 16)
  length = sqrt(25)
  length = 5

Step 2: Divide each component
  unit_x = 3 / 5 = 0.6
  unit_y = 4 / 5 = 0.8

Result: (0.6, 0.8)`}</code></pre>
            <p>Verify: <code>sqrt(0.6² + 0.8²) = sqrt(0.36 + 0.64) = sqrt(1) = 1</code> ✓</p>

            <h3>## Code</h3>
            <pre><code>{`fn normalize(v: Vec2) -> Vec2 {
    let length = (v.x * v.x + v.y * v.y).sqrt();
    if length == 0.0 {
        Vec2::ZERO
    } else {
        Vec2::new(v.x / length, v.y / length)
    }
}

// Most libraries have this built-in
let unit = vector.normalize();`}</code></pre>
          </section>

          <footer className="post-footer">
            <p>
              Built with Rust and Macroquad. Full source code available on{' '}
              <a href="https://github.com/fliqqs/2d_math_cheat_sheet" target="_blank" rel="noopener noreferrer">
                GitHub
              </a>.
            </p>
          </footer>
        </article>
      </main>
    </div>
  )
}

export default App

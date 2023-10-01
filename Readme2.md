# Install trunk and cargo-leptos

</br>
<code>
cargo install trunk cargo-leptos
</code>
</br>
</br>

# Setup tailwind

</br>1.Install tailwind:

<code>
npm install -D tailwindcss
npx tailwindcss init
</code>

</br>2. Modify tailwind.config.js:

<code>
 module.exports = {
 </br>content: {
     </br>  files: ["*.html", "./src/**/*.rs"],
 </br>},
 </br> ...
</code>

</br>3. Create input.css in a root of a project:

<code>
 </br>@tailwind base;
 </br>@tailwind components;
 </br>@tailwind utilities;
</code>

</br>4. Change file in Cargo.toml:

<code>
 </br>...
 </br>style-file = "style/output.css"
 </br>...
</code>

</br>5. Run the Tailwind process in separate terminal:

<code>
 </br>npx tailwindcss -i ./input.css -o ./style/output.css --watch
</code>
</br>
</br>


# Run project

</br>
<code>
cargo leptos watch
</code>
</br>
</br>

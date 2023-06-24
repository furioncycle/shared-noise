use leptos::*;
use leptos_router::*;

#[component]
pub fn Resume(cx: Scope) -> impl IntoView {
    view! {
        cx,
        
<div class="parallax">
            <h1>"Angel Marquez"</h1>
            <p><span class="box"> " "</span> "ttecho2021@gmail.com | "<span
            class="box">"  "</span> <a
            href="https://github.com/furioncycle">"github.com/furioncycle"</a> "|" <span
            class="box">"  "</span> <a href="ttecho.audio">"ttecho.audio"</a>" | "<span
            class="box">"  "</span> <a
            href="https://www.linkedin.com/in/angel-marquez-67863952">"me.linkedin"</a>
            "| "</p>
            <h2>"Work Experience"</h2>
        <div>
        <p><strong>"APR-39 Radar Warning Reciever - NAWCWD"</strong>"  2017/10 -
        11/2022"<br />"
        CI/CD Maintainer and Developer  Point Mugu, CA, USA"<br />
        </p>
        <ul>
        <li><p>"Develop a Go application that generates a Software Formal
        Qualification Test report from gitlab issues based on a future
        release"</p></li>
        <li><p>"Implement and maintain our CI pipeline in an offline GitLab
        server for the APR-39 that includes building, analyzing, testing, and
        packaging"</p></li>
        <li><p>"Port a legacy Visual C++ 6 custom packager for Operation Flight
        Program to Python to integrate into our CI build process"</p></li>
        </ul>
        </div>
        <div>
        <p><strong>"APR-39 Radar Warning Reciever - NAWCWD"</strong>"  2018/08 -
        11/2022"<br />
        "Lead Embedded Software Engineer  Point Mugu, CA, USA"<br />
        </p>
        <ul>
        <li><p>"Implement MIL-1553 standard messaging protocol for integration
        with mission critical systems"</p></li>
        <li><p>"Characterize and debug RF anomalies in Operational Flight Program
        (OFP) using Matlab"</p></li>
        <li><p>"Debug mission critical flash memory corruption due to customer
        mission data files"</p></li>
        <li><p>"Develop CRT display menus for systems on aircraft to interact
        with the system"</p></li>
        <li><p>"Review both Interface Control and Functional Requirement
        Documents to ensure system is within scope and compliance"</p></li>
        </ul>
        </div>
        <div>
        <p><strong>"Gorgon Automated Simulation Testbed -
        NAWCWD"</strong>"  2019/10 - 11/2022"<br />
        "Software Developer  Point Mugu, CA, USA"<br />
        </p>
        <ul>
        <li><p>"Mission Computer simulator in C#"<br />
        "to interact with in-house script environment used for integration
        testing with embedded systems"</p></li>
        <li><p>"Fix Offloader MQTT protocol acquiring RF data from the APR-39 for
        data analysis in Matlab"</p></li>
        </ul>
        </div>
        <h2>"Projects"</h2>
        <h2>"Education"</h2>
        <div>
        <p><strong>"Cal State University, Long Beach"</strong>"  2011/08 -
        2015/05"<br />
        "Bachelor of Science in Computer Engineering  Long Beach, CA, USA"<br />
        </p>
        </div>
        <div>
        <p><strong>"Cal State University, Chico"</strong>"  2004/08 - 2009/05"<br />
        "Bachelor of Art in Music Industry and Technology  Chico, CA, USA"<br />
        </p>
        </div>
        <h2>"Skills"</h2>
        <p><strong>"Programming Languages"</strong>" :: Ada, Bash, C++, C#, Python,
        Zig"<br />
        <strong>"Compilers/Build Tools"</strong>" :: Clang, CMake, Dotnet, GCC,
        GNAT, Make"<br />
        <strong>"IDE"</strong>" :: Visual Studio/VSCode, Windriver"<br />
        <strong>"Debuggers"</strong>" :: GDB, LLDB, Visual Studio Debugger,
        Windriver OnChip"<br />
        <strong>"Version Control"</strong>" :: Git"<br />
        </p>
        <div data-align="right">
        <p>"Last Update on April 25, 2023 using Typst"</p>
        </div>        
        </div>    
    }
}